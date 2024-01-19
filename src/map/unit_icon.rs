use yew::prelude::*;
use yew_autoprops::autoprops;

use super::ViewTransform;

#[autoprops]
#[function_component]
pub fn UnitIcon(
    id: usize,
    view: &ViewTransform,
) -> Html {
    use crate::data::{DataContext, DataAction};
    let data_ctx = use_context::<DataContext>().unwrap();

    let this_unit = data_ctx.units[id].clone();

    let is_selected = data_ctx.selected == Some(id);

    let name = this_unit.name().to_owned();

    let position = view.transform(this_unit.position().xz());

    let cx = (50.0 + position.x).to_string();
    let cy = (50.0 - position.y).to_string();

    let fill = if is_selected { "#f33" } else { "#900" };

    let onclick = move |_| data_ctx.dispatch(DataAction::Select(id));

    html!{
        <circle {onclick} {cx} {cy} r="1" {fill}>
            <title>{name}</title>
        </circle>
    }
}
