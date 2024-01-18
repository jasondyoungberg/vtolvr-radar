use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::Msg;
use crate::Unit;

#[autoprops]
#[function_component(UnitConfig)]
pub fn unit_config(
    unit: &Unit,
    id: usize,
    is_selected: bool,
    messenger: Callback<Msg>,
) -> Html {

    let delete_onclick = messenger.reform(move |_| Msg::DeleteUnit(id));

    html! {
        <div class={
            if is_selected {
                "unit selected"
            } else {
                "unit"
            }
        }>
            <p class="name">{ unit.name() }</p>
            <pre><code>{format!("{:?}", unit)}</code></pre>
            <button onclick={delete_onclick}>{"Delete"}</button>
        </div>
    }
}
