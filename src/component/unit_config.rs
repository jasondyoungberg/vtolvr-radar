use crate::{*, input::TextInput};

#[derive(Properties, PartialEq)]
pub struct UnitConfigProps {
    pub unit: Unit,
    pub id: usize,
    pub messenger: Callback<Msg>
}

pub struct UnitConfig {}

impl Component for UnitConfig {
    type Message = ();
    type Properties = UnitConfigProps;

    fn create(_ctx: &Context<Self>) -> Self {
        UnitConfig { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let id = props.id;
        let unit = props.unit.clone();

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
                        props.messenger.reform(move |txt| {
                            let mut new_unit = unit.clone();
                            new_unit.name = txt;
                            Msg::UpdateUnit(id, new_unit)
                        })
                    }
                />
                <button
                    onclick={
                        props.messenger.reform(move |_| Msg::DeleteUnit(id))
                    }
                >
                    {"Delete"}
                </button>
            </div>
        }
    }
}

/*
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
            <p class="name">{ &unit.name }</p>
            <pre><code>{format!("{:?}", unit)}</code></pre>
            <TextInput value={unit.name.clone()} on_change={name_handler} />
            <button onclick={delete_handler}>{"Delete"}</button>
        </div>
    }
}
*/
