use nalgebra::Vector3;

#[derive(Clone, Debug, PartialEq)]
pub struct Nmss {
    pub name: String,
    pub position: Vector3<f64>,
}
