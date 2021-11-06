pub struct Section {
    A: f64,
    Iy: f64,
    Iz: f64,
    K: f64,
}

impl Section {
    pub fn new(A: f64, Iy: f64, Iz: f64, K: f64) -> Self {
        Section { A, Iy, Iz, K }
    }

    pub fn get_A(&self) -> f64 {
        self.A
    }

    pub fn get_Iy(&self) -> f64 {
        self.Iy
    }

    pub fn get_Iz(&self) -> f64 {
        self.Iz
    }

    pub fn get_K(&self) -> f64 {
        self.K
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_instance() {
        use super::*;
        let section = Section::new(100.0, 1000.0, 10000.0, 0.0);
        assert_eq!(section.A, 100.0);
        assert_eq!(section.Iy, 1000.0);
        assert_eq!(section.Iz, 10000.0);
        assert_eq!(section.K, 0.0);
    }
}
