use yew::prelude::*;

mod component;
mod unit;
mod data;
mod scenario_map;

use component::UnitList;

#[function_component]
pub fn App() -> Html {
    use crate::data::DataProvider;
    use crate::scenario_map::ScenarioMap;

    log::info!("App");

    html!{
        <div class="main">
            <DataProvider>
                <ScenarioMap />
                <UnitList />
            </DataProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
