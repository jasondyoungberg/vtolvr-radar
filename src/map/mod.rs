use nalgebra::Vector2;
use yew::prelude::*;

mod unit_icon;

#[derive(Clone, Debug, PartialEq)]
pub struct ViewTransform {
    center: Vector2<f64>,
    scale: f64,
}

impl ViewTransform {
    pub fn transform(&self, v: Vector2<f64>) -> Vector2<f64> {
        (v - self.center) * self.scale
    }
}

#[function_component]
pub fn ScenarioMap() -> Html {
    use crate::data::DataContext;
    use unit_icon::UnitIcon;

    let data_ctx = use_context::<DataContext>().unwrap();

    let view = use_state(|| ViewTransform {
        center: Vector2::new(0.0, 0.0),
        scale: 1.0/1000.0,
    });

    html! {
        <div class="map">
            <svg
                viewBox="0 0 100 100">
                { data_ctx.units.iter().enumerate().map(|(i,_unit)| {
                    html!{
                        <UnitIcon id={i} view={(*view).clone()}/>
                    }
                }).collect::<Html>() }
            </svg>
        </div>
    }
}
