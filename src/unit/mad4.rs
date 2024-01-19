use nalgebra::Vector3;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Mad4 {
    pub name: String,
    pub position: Vector3<f64>,
}

impl Mad4 {
    pub fn icon(&self, color: &str) -> Html {
        let d = "M -8 7 L -10 6 L -10 5 L -7 3 L -6 3 L -6 5 L -3 5 L -5 4 L -5 3 L -4 3 L -4 -5 L -6 -5 L -6 -8 L -1 -8 L -1 -5 L -3 -5 L -3 3 L 9 3 L 7 -1 L 8 -1 L 10 3 L 10 4 L 8 5 L 10 5 L 10 7 L 9 7 L 8 8 L 7 7 L 6 8 L 5 7 L -3 7 L -4 8 L -5 7 L -6 8 L -7 7 Z";
        html! { <path {d} fill={color.to_owned()} /> }
    }
}
