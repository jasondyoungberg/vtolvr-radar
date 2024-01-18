use crate::component::UnitConfig;
use crate::Unit;
use crate::Msg;

use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(UnitList)]
pub fn unit_list(
    units: &Vec<Unit>,
    selected_id: Option<usize>,
    messenger: Callback<Msg>
) -> Html {
    html!{
        <div class="config">
            {units.iter().enumerate().map(|(i, unit)| {
                html!{
                    <UnitConfig 
                        unit={unit.clone()}
                        id={i}
                        is_selected={selected_id == Some(i)}
                        messenger={messenger.clone()}
                    />
                }
            }).collect::<Html>()}
        </div>
    }
}
