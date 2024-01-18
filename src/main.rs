use yew::prelude::*;
extern crate nalgebra as na;

use std::vec;

mod component;
mod unit;
mod input;

use component::*;
use unit::*;

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
                Unit {
                    name: "Unit 1".to_string(),
                    position: na::Vector3::new(0.0, 0.0, 0.0),
                },
                Unit {
                    name: "Unit 2".to_string(),
                    position: na::Vector3::new(0.0, 0.0, 0.0),
                },
                Unit {
                    name: "Unit 3".to_string(),
                    position: na::Vector3::new(0.0, 0.0, 0.0),
                },
            ],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <UnitList
                units={self.units.clone()}
                on_update={link.callback(|msg| msg)}
            />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        log::info!("Rendered App");
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
