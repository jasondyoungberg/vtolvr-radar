use nalgebra::Vector3;

mod f45; pub use f45::F45;
mod mad4; pub use mad4::Mad4;

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    TestUnit(TestUnit),
    F45(F45),
    Mad4(Mad4),
}

impl Unit {
    pub fn name(&self) -> &str {
        match self {
            Unit::TestUnit(unit) => &unit.name,
            Unit::F45(unit) => &unit.name,
            Unit::Mad4(unit) => &unit.name,
        }
    }

    pub fn position(&self) -> &Vector3<f64> {
        match self {
            Unit::TestUnit(unit) => &unit.position,
            Unit::F45(unit) => &unit.position,
            Unit::Mad4(unit) => &unit.position,  
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Unit::TestUnit(_) => "TestUnit",
            Unit::F45(_) => "F-45A",
            Unit::Mad4(_) => "MAD-4",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestUnit {
    pub name: String,
    pub position: Vector3<f64>,
}
