use yew::prelude::*;

mod component;
mod unit;

mod scenario_map;

pub use unit::Unit;

use component::UnitList;

#[derive(Debug)]
pub struct App {
    units: Vec<Unit>,
    selected: Option<usize>,
}

#[derive(Debug)]
pub enum Msg {
    DeleteUnit(usize),
    UpdateUnit(usize, Unit),
    AddUnit(Unit),
    Select(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Revieved message: {:?}", msg);

        match msg {
            Msg::DeleteUnit(i)       => { self.units.remove(i);  },
            Msg::UpdateUnit(i, unit) => { self.units[i] = unit;  },
            Msg::AddUnit(unit)       => { self.units.push(unit); },
            Msg::Select(i)           => { self.selected = Some(i); },
        };

        true
    }

    fn create(_ctx: &Context<Self>) -> Self {
        use unit::*;
        App {
            units: vec![
                Unit::TestUnit(TestUnit {
                    name: "Test".to_string(),
                    position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
                }),
                Unit::F45(F45 {
                    name: "Fighter".to_string(),
                    position: nalgebra::Vector3::new(
                        3_000.0, 5_000.0, 15_000.0,
                    ),
                }),
                Unit::Mad4(Mad4 {
                    name: "Radar".to_string(),
                    position: nalgebra::Vector3::new(
                        -2_000.0, 100.0, -10_000.0,
                    ),
                }),
            ],
            selected: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use scenario_map::ScenarioMap;

        let messenger = ctx.link().callback(|msg| msg);

        html! {
            <div class="main">
                <ScenarioMap
                    units={self.units.clone()}
                    messenger={messenger.clone()}
                />
                <UnitList
                    units={self.units.clone()}
                    selected_id={self.selected}
                    messenger={messenger.clone()}
                />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
