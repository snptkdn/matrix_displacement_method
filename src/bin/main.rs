use matrix_displacement_method::element::node::*;
use matrix_displacement_method::element::section::*;
use matrix_displacement_method::element::material::*;
use matrix_displacement_method::element::element::*;
use matrix_displacement_method::element::element::*;
use matrix_displacement_method::element::beam::*;
use matrix_displacement_method::element::pt::*;
use nalgebra as na;
fn main() {
    let node1 = Node::new(1, Pt::new(1.0, 0.0, 0.0));
    let node2 = Node::new(2, Pt::new(2.0, 0.0, 0.0));
    let section = Section::new(100.0, 1000.0, 10000.0, 0.0, 1.0);
    let material = Material::default();
    let count = 2;

    let mut node_matrix = na::DMatrix::from_element(6*count, 6*count, 0.0);
    let beam = Beam::new(1, &node1, &node2, &section, &material);

    let matrix = beam.get_rigid_matrix();

    println!("{}", matrix);
}
