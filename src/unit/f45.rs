use nalgebra::Vector3;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct F45 {
    pub name: String,
    pub position: Vector3<f64>,
}

impl F45 {
    pub fn icon(&self) -> Html {
        let heading = 0.0f64;//self.rotation.euler_angles().2;
        let transform = format!("rotate({})", heading.to_degrees());

        html!{
            <image href="img/f45.png" {transform}
            x="-50" y="-50" width="100" height="100" />
        }
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
