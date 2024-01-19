use yew::prelude::*;

use crate::config::UnitOptions;

mod unit_options;
pub use unit_options::*;

#[function_component]
pub fn ConfigPanel() -> Html {
    use crate::data::{DataContext, DataAction};
    use crate::unit::Unit;

    let data_ctx = use_context::<DataContext>().unwrap();

    let new_onclick = {
        let data_ctx = data_ctx.clone();
        let seed = data_ctx.units.len() as u64;
        move |_| data_ctx.dispatch(DataAction::Add(Unit::random(seed)))
    };

    html!{
        <div class="config">
            <button onclick={new_onclick}>{"New Unit"}</button>
            {data_ctx.units.iter().enumerate().map(|(i, _unit)| {
                html!{<UnitOptions id={i}/>}
            }).collect::<Html>()}
        </div>
    }
}
