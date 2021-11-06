use super::pt::*;
#[derive(Debug)]
pub struct Node {
    num: i32,
    pt: Pt,
}

impl Node {
    pub fn new(num: i32, pt: Pt) -> Self {
        Node { num, pt }
    }

    fn get_pt(&self) -> &Pt {
        &self.pt
    }

    pub fn get_distance_to(&self, tar: &Node) -> f64 {
        self.pt.get_distace_to(tar.get_pt())
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, tar: &Node) -> bool {
        &self.num == &tar.num
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn get_distance() {
        use super::*;
        let node1 = Node::new(1, Pt::new(1.0, 4.0, 1.0));
        let node2 = Node::new(2, Pt::new(5.0, 7.0, 1.0));
        assert_eq!(5.0, node1.get_distance_to(&node2));
    }
}
