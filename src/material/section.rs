pub struct Section {
    A: f32,
    Iy: f32,
    Iz: f32,
    K: f32,
}

impl Section {
    pub fn new(A: f32, Iy: f32, Iz: f32, K: f32) -> Section {
        Section { A, Iy, Iz, K }
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
