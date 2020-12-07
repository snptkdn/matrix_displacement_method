// TODO:should impl GenericType
#[derive(Debug)]
pub struct Pt{
    x: f64,
    y: f64
}

impl Pt    
{
    pub fn new(x: f64, y: f64) -> Self {
        Pt{
            x,
            y
        }
    }

    pub fn get_distace_to(&self, tar: &Self) -> f64{
        ((self.x - tar.x).powf(2f64) + (self.y - tar.y).powf(2f64)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_distance() {
        use super::*;
        let pt1 = Pt::new(1.0, 2.0);
        let pt2 = Pt::new(4.0, 6.0);
        let dis = pt1.get_distace_to(&pt2);
        assert_eq!(5.0 , dis);
    }
}