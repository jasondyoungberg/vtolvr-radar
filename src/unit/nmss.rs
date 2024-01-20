use nalgebra::Vector3;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Nmss {
    pub name: String,
    pub position: Vector3<f64>,
}

impl Nmss {
    pub fn icon(&self) -> Html {
        html! { <circle cx="0" cy="0" r="50" fill="#999"/> }
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
