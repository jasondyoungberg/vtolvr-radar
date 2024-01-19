use crate::config::UnitOptions;

use yew::prelude::*;

#[function_component]
pub fn ConfigPanel() -> Html {
    use crate::data::DataContext;
    let data_ctx = use_context::<DataContext>().unwrap();

    html!{
        <div class="config">
            {data_ctx.units.iter().enumerate().map(|(i, _unit)| {
                html!{<UnitOptions id={i}/>}
            }).collect::<Html>()}
        </div>
    }
}
