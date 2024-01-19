use nalgebra::Vector2;
use yew::prelude::*;

#[function_component(ScenarioMap)]
pub fn scenario_map() -> Html {
    use crate::data::{DataContext, DataAction};
    let data_ctx = use_context::<DataContext>().unwrap();

    let center = Vector2::new(0.0, 0.0);
    let scale = 1.0/1000.0;

    html! {
        <div class="map">
            <svg
                viewBox="0 0 100 100">
                { data_ctx.units.iter().enumerate().map(|(i,unit)| {

                    let name = unit.name().to_owned();

                    let position = unit.position().xz();
                    let position = (position - center) * scale;

                    let cx = (50.0 + position.x).to_string();
                    let cy = (50.0 - position.y).to_string();

                    let onclick = {
                        let data_ctx = data_ctx.clone();
                        move |_| data_ctx.dispatch(DataAction::SelectUnit(i))
                    };

                    html!{
                        <circle {onclick} {cx} {cy} r="1" fill="red">
                            <title>{name}</title>
                        </circle>
                    }
                }).collect::<Html>() }
            </svg>
        </div>
    }
}
