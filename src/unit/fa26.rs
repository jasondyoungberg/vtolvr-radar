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

impl rand::distributions::Distribution<Fa26> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Fa26 {
        use rand::random;

        let name = format!("Random {}", random::<u16>());
        let position = Vector3::new(
            rng.gen_range( -40_000.0 .. 40_000.0 ),
            rng.gen_range(       0.0 .. 10_000.0 ),
            rng.gen_range( -40_000.0 .. 40_000.0 ),
        );

        Fa26 { name, position }
    }
}
