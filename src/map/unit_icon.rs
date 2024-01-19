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

    let transform = format!("translate({} {}) scale(0.2)",
        (50.0 + position.x), (50.0 - position.y));

    let fill = if is_selected { "#f33" } else { "#900" };

    let onclick = move |_| data_ctx.dispatch(DataAction::Select(id));

    html!{
        <g {onclick} {transform}>
            {this_unit.icon(fill)}
            <title>{name}</title>
        </g>
    }
}
