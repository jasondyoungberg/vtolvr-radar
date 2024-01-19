use nalgebra::Vector3;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct F45 {
    pub name: String,
    pub position: Vector3<f64>,
}

impl F45 {
    pub fn icon(&self, color: &str) -> Html {
        let d = "M 0 -10 L 2 -6 L 2 -4 L 6 0 L 2 0 L 10 8 L 2 8 L 4 10 L 2 10 L 0 8 L -2 10 L -4 10 L -2 8 L -10 8 L -2 0 L -6 0 L -2 -4 L -2 -6 Z";
        html! { <path {d} fill={color.to_owned()} /> }
    }
}

impl rand::distributions::Distribution<F45> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> F45 {
        use rand::random;

        let name = format!("Random {}", random::<u16>());
        let position = Vector3::new(
            rng.gen_range( -40_000.0 .. 40_000.0 ),
            rng.gen_range(       0.0 .. 10_000.0 ),
            rng.gen_range( -40_000.0 .. 40_000.0 ),
        );

        F45 { name, position }
    }
}
