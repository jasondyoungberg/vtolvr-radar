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

impl rand::distributions::Distribution<Nmss> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Nmss {
        use rand::random;

        let name = format!("Random {}", random::<u16>());
        let position = Vector3::new(
            rng.gen_range( -40_000.0 .. 40_000.0 ),
            0.0,
            rng.gen_range( -40_000.0 .. 40_000.0 ),
        );

        Nmss { name, position }
    }
}
