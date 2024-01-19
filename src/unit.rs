use nalgebra::Vector3;

mod f45; pub use f45::F45;
mod mad4; pub use mad4::Mad4;
use rand::random;

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    F45(F45),
    Mad4(Mad4),
}

impl Unit {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let name = format!("Random {}", random::<u16>());
        let position = Vector3::new(
            rng.gen_range( -40_000.0 .. 40_000.0 ),
            rng.gen_range(       0.0 .. 10_000.0 ),
            rng.gen_range( -40_000.0 .. 40_000.0 ),
        );

        match rng.gen_range(0..2) {
            0 => Unit::F45(F45 { name, position }),
            1 => Unit::Mad4(Mad4 { name, position }),
            _ => unreachable!(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Unit::F45(unit) => &unit.name,
            Unit::Mad4(unit) => &unit.name,
        }
    }

    pub fn position(&self) -> &Vector3<f64> {
        match self {
            Unit::F45(unit) => &unit.position,
            Unit::Mad4(unit) => &unit.position,  
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Unit::F45(_) => "F-45A",
            Unit::Mad4(_) => "MAD-4",
        }
    }
}
