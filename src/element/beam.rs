use super::super::Edge;
use super::element::*;
use super::material::*;
use super::node::*;
use super::section::*;
use nalgebra as na;

pub struct Beam<'a> {
    num: i32,
    i: &'a Node,
    j: &'a Node,
    section: &'a Section,
    material: &'a Material,
}

impl<'a> Beam<'a> {
    pub fn new(
        num: i32,
        i: &'a Node,
        j: &'a Node,
        section: &'a Section,
        material: &'a Material,
    ) -> Self {
        Self {
            num,
            i,
            j,
            section,
            material,
        }
    }

    pub fn get_node(&self, edge: Edge) -> &Node {
        match edge {
            Edge::EdgeI => &self.i,
            Edge::EdgeJ => &self.j,
            _ => panic!("cant take a node."),
        }
    }
}

impl<'a> Element for Beam<'a> {
    fn get_rigid_matrix(&self) -> RigidMatrix {
        let (A, Iy, Iz, K, J, E, G, l) = (
            self.get_section().get_A(),
            self.get_section().get_Iy(),
            self.get_section().get_Iz(),
            self.get_section().get_K(),
            self.get_section().get_J(),
            self.get_material().get_E(),
            self.get_material().get_G(),
            self.get_length(),
        );

        RigidMatrix::from_row_slice
        (&[
             E*A/l,                  0.0,                  0.0,    0.0,                 0.0,                0.0, -E*A/l,                  0.0,                  0.0,    0.0,                 0.0,                0.0,
               0.0,            12.0*E*Iz,                  0.0,    0.0,                 0.0, 6.0*E*Iz/l.powi(2),    0.0, -12.0*E*Iz/l.powi(3),                  0.0,    0.0,                 0.0, 6.0*E*Iz/l.powi(2),
               0.0,                  0.0,  12.0*E*Iy/l.powi(3),    0.0, -6.0*E*Iy/l.powi(2),                0.0,    0.0,                  0.0, -12.0*E*Iy/l.powi(3),    0.0, -6.0*E*Iy/l.powi(2),                0.0,
               0.0,                  0.0,                  0.0,  G*J/l,                 0.0,                0.0,    0.0,                  0.0,                  0.0, -G*J/l,                 0.0,                0.0,
               0.0,                  0.0,  -6.0*E*Iy/l.powi(2),    0.0,          4.0*E*Iy/l,                0.0,    0.0,                  0.0,   6.0*E*Iy/l.powi(2),    0.0,          2.0*E*Iy/l,                0.0,
               0.0,  -6.0*E*Iz/l.powi(2),                  0.0,    0.0,                 0.0,         4.0*E*Iz/l,    0.0,  -6.0*E*Iz/l.powi(2),                  0.0,    0.0,                 0.0,         2.0*E*Iz/l,

            -E*A/l,                  0.0,                  0.0,    0.0,                 0.0,                0.0,  E*A/l,                  0.0,                  0.0,    0.0,                 0.0,                0.0,
               0.0, -12.0*E*Iz/l.powi(3),                  0.0,    0.0,                 0.0, 6.0*E*Iz/l.powi(2),    0.0,            12.0*E*Iz,                  0.0,    0.0,                 0.0, 6.0*E*Iz/l.powi(2),
               0.0,                  0.0, -12.0*E*Iy/l.powi(3),    0.0, -6.0*E*Iy/l.powi(2),                0.0,    0.0,                  0.0,  12.0*E*Iy/l.powi(3),    0.0, -6.0*E*Iy/l.powi(2),                0.0,
               0.0,                  0.0,                  0.0, -G*J/l,                 0.0,                0.0,    0.0,                  0.0,                  0.0,  G*J/l,                 0.0,                0.0,
               0.0,                  0.0,   6.0*E*Iy/l.powi(2),    0.0,          2.0*E*Iy/l,                0.0,    0.0,                  0.0,  -6.0*E*Iy/l.powi(2),    0.0,          4.0*E*Iy/l,                0.0,
               0.0,  -6.0*E*Iz/l.powi(2),                  0.0,    0.0,                 0.0,         2.0*E*Iz/l,    0.0,  -6.0*E*Iz/l.powi(2),                  0.0,    0.0,                 0.0,         4.0*E*Iz/l,
        ])
    }

    fn get_length(&self) -> f64 {
        self.i.get_distance_to(self.j)
    }

    fn get_section(&self) -> &Section {
        self.section
    }

    fn get_material(&self) -> &Material {
        self.material
    }
}

#[cfg(test)]
mod tests {
    use super::super::pt::*;
    #[test]
    fn get_node() {
        use super::*;
        let node_i = Node::new(1, Pt::new(1.0, 4.0, 1.0));
        let node_j = Node::new(2, Pt::new(5.0, 7.0, 1.0));
        let section = Section::new(100.0, 10000.0, 10000.0, 0.0, 1.0);
        let material: Material = Default::default();
        let beam = Beam::new(1, &node_i, &node_j, &section, &material);
        assert_eq!(&node_i, beam.get_node(Edge::EdgeI));
        assert_eq!(&node_j, beam.get_node(Edge::EdgeJ));
        assert_eq!(5.0, beam.get_length());
    }
}
