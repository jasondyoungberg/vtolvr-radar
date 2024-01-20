use nalgebra::Vector3;

mod f45; pub use f45::F45;
mod fa26; pub use fa26::Fa26;
mod mad4; pub use mad4::Mad4;
mod nmss; pub use nmss::Nmss;

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Fa26(Fa26),
    F45 (F45 ),
    Mad4(Mad4),
    Nmss(Nmss),
}

impl Unit {
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

    pub fn icon(&self) -> yew::Html {
        match self {
            Unit::Fa26(unit) => unit.icon(),
            Unit::F45 (unit) => unit.icon(),
            Unit::Mad4(unit) => unit.icon(),
            Unit::Nmss(unit) => unit.icon(),
        }
    }
}

impl rand::distributions::Distribution<Unit> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Unit {
        use rand::random;

        match rng.gen_range(0..4) {
            0 => Unit::Fa26(random()),
            1 => Unit::F45 (random()),
            2 => Unit::Mad4(random()),
            3 => Unit::Nmss(random()),
            _ => unreachable!(),
        }
    }
}
