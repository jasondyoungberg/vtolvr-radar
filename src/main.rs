use yew::prelude::*;

mod convert;
mod config;
mod unit;
mod data;
mod map;

mod input;

use config::ConfigPanel;

#[function_component]
pub fn App() -> Html {
    use crate::data::DataProvider;
    use crate::map::ScenarioMap;

    log::info!("App");

    html!{
        <div class="main">
            <DataProvider>
                <ScenarioMap />
                <ConfigPanel />
            </DataProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
