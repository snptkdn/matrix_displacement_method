/// E is young's modulus
/// G is elastic shear modulus
pub struct Material {
    E: f64,
    G: f64,
}

impl Material {
    pub fn new(E: f64, G: f64) -> Self {
        Material { E, G }
    }

    pub fn get_E(&self) -> f64 {
        self.E
    }

    pub fn get_G(&self) -> f64 {
        self.G
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            E: 20600.0,
            G: 7923.0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_instance() {
        use super::*;
        let material: Material = Default::default();
        assert_eq!(material.E, 20600.0);
        assert_eq!(material.G, 7923.0);
    }
}
