use crate::component::UnitConfig;

use yew::prelude::*;

#[function_component]
pub fn UnitList() -> Html {
    use crate::data::DataContext;
    let data_ctx = use_context::<DataContext>().unwrap();

    html!{
        <div class="config">
            {data_ctx.units.iter().enumerate().map(|(i, unit)| {
                html!{
                    <UnitConfig id={i}/>
                }
            }).collect::<Html>()}
        </div>
    }
}
