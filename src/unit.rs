use nalgebra::{Vector3, UnitQuaternion};

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    pub name: String,
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub rotation: UnitQuaternion<f64>,
}

impl Unit {
    pub fn new(name: &str) -> Self {
        Unit {
            name: name.to_string(),
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::identity(),
        }
    }
}
