use nalgebra::{Vector3, UnitQuaternion};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Fa26 {
    pub name: String,
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub rotation: UnitQuaternion<f64>,
}

impl Fa26 {
    pub fn icon(&self) -> Html {
        let heading = self.rotation.euler_angles().2;

        let transform = format!("rotate({})", heading.to_degrees());
        
        html!{
            <image href="img/fa26.png" {transform}
            x="-50" y="-50" width="100" height="100" />
        }
    }
}

impl rand::distributions::Distribution<Fa26> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Fa26 {
        use rand::random;
        use std::f64::consts::TAU;

        let name = format!("Random {}", random::<u16>());
        let position = Vector3::new(
            rng.gen_range( -40_000.0 .. 40_000.0 ),
            rng.gen_range(       0.0 .. 10_000.0 ),
            rng.gen_range( -40_000.0 .. 40_000.0 ),
        );
        let velocity = Vector3::new(
            rng.gen_range( -500.0 .. 500.0 ),
            rng.gen_range( -500.0 .. 500.0 ),
            rng.gen_range( -500.0 .. 500.0 ),
        );
        let rotation = UnitQuaternion::from_euler_angles(
            rng.gen_range( 0.0 .. TAU ),
            rng.gen_range( 0.0 .. TAU ),
            rng.gen_range( 0.0 .. TAU ),
        );

        Fa26 { name, position, velocity, rotation }
    }
}
