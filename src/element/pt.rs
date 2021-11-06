// TODO:should impl GenericType
#[derive(Debug)]
pub struct Pt {
    x: f64,
    y: f64,
    z: f64,
}

impl Pt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Pt { x, y, z }
    }

    pub fn get_distace_to(&self, tar: &Self) -> f64 {
        ((self.x - tar.x).powf(2f64) + (self.y - tar.y).powf(2f64) + (self.z - tar.z).powf(2f64))
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_distance() {
        use super::*;
        let pt1 = Pt::new(1.0, 2.0, 1.0);
        let pt2 = Pt::new(4.0, 6.0, 1.0);
        let dis1 = pt1.get_distace_to(&pt2);
        let dis2 = pt2.get_distace_to(&pt1);
        assert_eq!(5.0, dis1);
        assert_eq!(5.0, dis2);
    }
}
