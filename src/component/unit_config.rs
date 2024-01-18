use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::component::TextInput;
use crate::Msg;
use crate::Unit;

#[autoprops]
#[function_component(UnitConfig)]
pub fn unit_config(
    unit: &Unit,
    id: usize,
    messenger: Callback<Msg>,
) -> Html {
    html! {
        <div class="unit">
            <p class="name">{ &unit.name }</p>
            <p class="position">
                {format!("Position: ({:.2}, {:.2}, {:.2})",
                    unit.position.x, unit.position.y, unit.position.z)}
            </p>
            <p class="velocity">
                {format!("Velocity: ({:.2}, {:.2}, {:.2})",
                    unit.velocity.x, unit.velocity.y, unit.velocity.z)}
            </p>
            <p class="rotation">
                {format!("Rotation: ({:.2}, {:.2}, {:.2}, {:.2})",
                    unit.rotation.coords.x, unit.rotation.coords.y,
                    unit.rotation.coords.z, unit.rotation.coords.w)}
            </p>

            <pre><code>{format!("{:?}", unit)}</code></pre>
            <TextInput
                value={unit.name.clone()}
                on_change={
                    let unit = unit.clone();
                    messenger.reform(move |txt| {
                        let mut new_unit = unit.clone();
                        new_unit.name = txt;
                        Msg::UpdateUnit(id, new_unit)
                    })
                }
            />
            <button
                onclick={
                    messenger.reform(move |_| Msg::DeleteUnit(id))
                }
            >
                {"Delete"}
            </button>
        </div>
    }
}
