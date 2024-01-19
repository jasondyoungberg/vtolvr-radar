use nalgebra::Vector3;

mod f45; pub use f45::F45;
mod fa26; pub use fa26::Fa26;
mod mad4; pub use mad4::Mad4;
mod nmss; pub use nmss::Nmss;

use rand::random;

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Fa26(Fa26),
    F45 (F45 ),
    Mad4(Mad4),
    Nmss(Nmss),
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

        match rng.gen_range(0..4) {
            0 => Unit::Fa26(Fa26 { name, position }),
            1 => Unit::F45 (F45  { name, position }),
            2 => Unit::Mad4(Mad4 { name, position }),
            3 => Unit::Nmss(Nmss { name, position }),
            _ => unreachable!(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Unit::Fa26(unit) => &unit.name,
            Unit::F45 (unit) => &unit.name,
            Unit::Mad4(unit) => &unit.name,
            Unit::Nmss(unit) => &unit.name,
        }
    }

    pub fn position(&self) -> &Vector3<f64> {
        match self {
            Unit::Fa26(unit) => &unit.position,
            Unit::F45 (unit) => &unit.position,
            Unit::Mad4(unit) => &unit.position,  
            Unit::Nmss(unit) => &unit.position,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Unit::Fa26(_) => "F/A-26B",
            Unit::F45 (_) => "F-45A",
            Unit::Mad4(_) => "MAD-4",
            Unit::Nmss(_) => "NMSS",
        }
    }

    pub fn icon(&self, color: &str) -> yew::Html {
        match self {
            Unit::Fa26(unit) => unit.icon(color),
            Unit::F45 (unit) => unit.icon(color),
            Unit::Mad4(unit) => unit.icon(color),
            Unit::Nmss(unit) => unit.icon(color),
        }
    }
}
