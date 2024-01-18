use yew::prelude::*;
extern crate nalgebra as na;

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    pub name: String,
    pub position: na::Vector3<f64>,
}

#[derive(Properties, PartialEq)]
pub struct UnitConfigProps {
    pub unit: Unit,
    pub on_update: Callback<Option<Unit>>
}

#[function_component(UnitConfig)]
pub fn unit_config(UnitConfigProps { unit, on_update }: &UnitConfigProps) -> Html {
    use crate::input::TextInput;

    let name_handler = {
        let on_update = on_update.clone();
        let unit = unit.clone();
        Callback::from(move |txt| {
            let mut new_unit = unit.clone();
            new_unit.name = txt;
            on_update.emit(Some(new_unit));
        })
    };

    let delete_handler = {
        let on_update = on_update.clone();
        Callback::from(move |_| {
            on_update.emit(None);
        })
    };

    html! {
        <div class="unit">
            <h3>{ &unit.name }</h3>
            <pre><code>{format!("{:?}", unit)}</code></pre>
            <TextInput value={unit.name.clone()} on_change={name_handler} />
            <button onclick={delete_handler}>{"Delete"}</button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct UnitListProps {
    pub units: Vec<Unit>,
    pub on_update: Callback<(usize, Option<Unit>)>
}

#[function_component(UnitList)]
pub fn unit_list(UnitListProps { units, on_update }: &UnitListProps) -> Html {
    units.iter().enumerate().map(|(i, unit)| {
        let update_handler = {
            let on_update = on_update.clone();
            Callback::from(move |unit: Option<Unit>| {
                on_update.emit((i, unit));
            })
        };

        html!{
            <>
                <UnitConfig unit={unit.clone()} on_update={update_handler}/>
            </>
        }
    }).collect::<Html>()
}
