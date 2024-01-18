use crate::UnitConfig;
use crate::Unit;
use crate::Msg;

use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component(UnitList)]
pub fn unit_list(
    units: &Vec<Unit>,
    messenger: Callback<Msg>
) -> Html {
    units.iter().enumerate().map(|(i, unit)| {
        html!{<UnitConfig 
            unit={unit.clone()}
            id={i}
            messenger={messenger.clone()}/>
        }
    }).collect::<Html>()
}
