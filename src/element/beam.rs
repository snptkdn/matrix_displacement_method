use super::super::Edge;
use super::node::*;
use super::section::*;
use super::material::*;
pub struct Beam<'a> {
    num: i32,
    i: &'a Node,
    j: &'a Node,
    section: &'a Section,
    material: &'a Material,
}

impl<'a> Beam<'a> {
    pub fn new(num: i32, i: &'a Node, j: &'a Node, section: &'a Section, material: &'a Material) -> Self {
        Self { num, i, j, section, material }
    }

    pub fn get_node(&self, edge: Edge) -> &Node {
        match edge {
            Edge::EdgeI => &self.i,
            Edge::EdgeJ => &self.j,
            _ => panic!("cant take a node.")
            }
    }

    pub fn get_length(&self) -> f64{
        self.i.get_distance_to(self.j)
    }
}

#[cfg(test)]
mod tests {
    use super::super::pt::*;
    #[test]
    fn get_node() {
        use super::*;
        let node_i = Node::new(1, Pt::new(1.0, 4.0,1.0));
        let node_j = Node::new(2, Pt::new(5.0, 7.0,1.0));
        let section = Section::new(100.0, 10000.0, 10000.0, 0.0);
        let material: Material = Default::default();
        let beam = Beam::new(1, &node_i, &node_j, &section, &material);
        assert_eq!(&node_i, beam.get_node(Edge::EdgeI));
        assert_eq!(&node_j, beam.get_node(Edge::EdgeJ));
        assert_eq!(5.0, beam.get_length());
    }
}
