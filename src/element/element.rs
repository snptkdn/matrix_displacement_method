use super::section::*;
use super::material::*;
use nalgebra as na;

const SIZE_MATRIX: i32 = 12;
pub type RigidMatrix = na::SMatrix<f64, 12, 12>;

pub trait Element {
    fn get_section(&self) -> &Section;

    fn get_material(&self) -> &Material;

    fn get_rigid_matrix(&self) -> RigidMatrix;

    fn get_length(&self) -> f64;
}