use yew::prelude::*;
extern crate nalgebra as na;

use std::vec;

mod unit;
mod input;
use unit::*;


#[function_component]
fn App() -> Html {
    let state = use_state(|| vec![
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
    ]);

    let on_update = {
        let state = state.clone();
        Callback::from(move |(i, unit): (usize, Option<Unit>)| {
            let mut new_state = state.to_vec();
            match unit {
                Some(unit) => {
                    new_state[i] = unit;
                },
                None => {
                    new_state.remove(i);
                }
            }
            state.set(new_state);
        })
    };

    html! {
        <UnitList units={state.to_vec()} on_update={on_update} />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
