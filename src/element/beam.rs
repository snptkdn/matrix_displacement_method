use super::super::Edge;
use super::node::*;
use super::section::*;
pub struct Beam<'a> {
    num: i32,
    i: &'a Node,
    j: &'a Node,
    section : Section,
}

impl<'a> Beam<'a> {
    pub fn new(num: i32, i: &'a Node, j: &'a Node, section: Section) -> Self {
        Self { num, i, j, section }
    }

    pub fn get_node(&self, edge: Edge) -> &Node {
        match edge {
            Edge::edge_i => &self.i,
            Edge::edge_j => &self.j,
            _ => panic!("cant take a node.")
            }
    }
}

#[cfg(test)]
mod tests {
    use super::super::pt::*;
    #[test]
    fn get_node() {
        use super::*;
        let node_i = Node::new(1, Pt::new(1.0, 4.0));
        let node_j = Node::new(2, Pt::new(5.0, 7.0));
        let section = Section::new(100.0, 10000.0, 10000.0, 0.0);
        let beam = Beam::new(1, &node_i, &node_j, section);
        assert_eq!(&node_i, beam.get_node(Edge::edge_i));
        assert_eq!(&node_j, beam.get_node(Edge::edge_j));
    }
}
