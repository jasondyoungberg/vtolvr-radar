use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub fn UnitOptions(
    id: usize,
) -> Html {
    use crate::data::{DataContext, DataAction};

    let data_ctx = use_context::<DataContext>().unwrap();

    let delete_onclick = {
        let data_ctx = data_ctx.clone();
        move |_| data_ctx.dispatch(DataAction::Delete(id))
    };
    let select_onclick = {
        let data_ctx = data_ctx.clone();
        move |_| data_ctx.dispatch(DataAction::Select(id))
    };

    let this_unit = data_ctx.units[id].clone();

    let is_selected = data_ctx.selected == Some(id);

    html! {
        <div
            class={if is_selected {"unit selected"} else {"unit"}}
            onclick={select_onclick}
        >
            <p class="name">
                { this_unit.name() }{" "}
                <span style="color: #ccc">
                    {"("}{ this_unit.type_name() }{")"}
                </span>
            </p>
            <pre><code>{format!("{:?}", data_ctx.units[id])}</code></pre>
            <button onclick={delete_onclick}>{"Delete"}</button>
        </div>
    }
}
