use nalgebra::Vector3;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Fa26 {
    pub name: String,
    pub position: Vector3<f64>,
}

impl Fa26 {
    pub fn icon(&self, color: &str) -> Html {
        let d = "M 0 -10 L 10 10 L -10 10 Z";
        html! { <path {d} fill={color.to_owned()} /> }
    }
}
