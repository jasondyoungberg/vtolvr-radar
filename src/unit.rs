use yew::prelude::*;
extern crate nalgebra as na;

use crate::Msg;

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    pub name: String,
    pub position: na::Vector3<f64>,
}

#[derive(Properties, PartialEq)]
pub struct UnitConfigProps {
    pub unit: Unit,
    pub id: usize,
    pub on_update: Callback<Msg>
}

#[function_component(UnitConfig)]
pub fn unit_config(UnitConfigProps { unit, id, on_update }: &UnitConfigProps) -> Html {
    use crate::input::TextInput;

    let name_handler = {
        let on_update = on_update.clone();
        let unit = unit.clone();
        let id = *id;
        Callback::from(move |txt| {
            let mut new_unit = unit.clone();
            let id = id.clone();
            new_unit.name = txt;
            on_update.emit(
                Msg::UpdateUnit(id, new_unit)
            );
        })
    };

    let delete_handler = {
        let on_update = on_update.clone();
        let id = *id;
        Callback::from(move |_| {
            on_update.emit(
                Msg::DeleteUnit(id)
            );
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
    pub on_update: Callback<Msg>
}

#[function_component(UnitList)]
pub fn unit_list(UnitListProps { units, on_update }: &UnitListProps) -> Html {
    units.iter().enumerate().map(|(i, unit)| {
        let update_handler = {
            let on_update = on_update.clone();
            Callback::from(move |msg| {
                on_update.emit(msg);
            })
        };

        html!{
            <>
                <UnitConfig unit={unit.clone()} id={i} on_update={update_handler}/>
            </>
        }
    }).collect::<Html>()
}
