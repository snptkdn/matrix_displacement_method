/// E is young's modulus
/// G is elastic shear modulus
pub struct Material {
    E: f32,
    G: f32,
}

impl Material {
    pub fn new(E: f32, G: f32) -> Self {
        Material { E, G }
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
