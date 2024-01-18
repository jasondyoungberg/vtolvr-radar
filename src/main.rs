use yew::prelude::*;

mod component;
mod unit;

pub use unit::Unit;

use component::UnitList;

#[derive(Debug)]
pub struct App {
    units: Vec<Unit>,
}

#[derive(Debug)]
pub enum Msg {
    DeleteUnit(usize),
    UpdateUnit(usize, Unit),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Revieved message: {:?}", msg);

        match msg {
            Msg::DeleteUnit(i) => {
                self.units.remove(i);
                true
            },
            Msg::UpdateUnit(i, unit) => {
                self.units[i] = unit;
                true
            }
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            units: vec![
                Unit::new("Alpha"),
                Unit::new("Bravo"),
                Unit::new("Charlie"),
                Unit::new("Delta"),
            ],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <UnitList
                units={self.units.clone()}
                messenger={link.callback(|msg| msg)}
            />
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
