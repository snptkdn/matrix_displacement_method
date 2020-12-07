use super::pt::*;
pub struct Node{
    pt : Pt,
}

impl Node{
    pub fn new(pt: Pt) -> Self{
        Node{pt}
    }

    fn get_pt(&self) -> &Pt{
        &self.pt
    }

    fn get_distance_to(&self, tar: Node) -> f64{
        self.pt.get_distace_to(tar.get_pt())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_distance() {
        use super::*;
        let node1 = Node::new(Pt::new(1.0, 4.0));
        let node2 = Node::new(Pt::new(5.0, 7.0));
        assert_eq!(5.0 , node1.get_distance_to(node2));
    }
}