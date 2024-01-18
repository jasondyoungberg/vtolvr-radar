use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    pub name: String,
    pub position: na::Vector3<f64>,
}
