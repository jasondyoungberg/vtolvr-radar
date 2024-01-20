use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::map::ViewTransform;

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


    let onclick = move |_| data_ctx.dispatch(DataAction::Select(id));
    let class = if is_selected { "unit-icon selected" } else { "unit-icon" };
    let transform = format!("translate({} {}) scale(0.1)",
        (50.0 + position.x), (50.0 - position.y));

    html!{
        <g {transform} {class}>
            {this_unit.icon()}
            <circle r=50 opacity=0 {onclick}/>
            <title>{name}</title>
        </g>
    }
}
