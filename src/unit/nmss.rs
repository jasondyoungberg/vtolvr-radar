use nalgebra::Vector3;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Nmss {
    pub name: String,
    pub position: Vector3<f64>,
}

impl Nmss {
    pub fn icon(&self, color: &str) -> Html {
        let d = "M -10 0 L 10 0 L 0 5 Z";
        html! { <path {d} fill={color.to_owned()} /> }
    }
}
