use crate::*;

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
