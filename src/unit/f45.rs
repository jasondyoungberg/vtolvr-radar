use nalgebra::Vector3;

#[derive(Clone, Debug, PartialEq)]
pub struct F45 {
    pub name: String,
    pub position: Vector3<f64>,
}
