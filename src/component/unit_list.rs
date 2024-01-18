use crate::*;

#[derive(Properties, PartialEq)]
pub struct UnitListProps {
    pub units: Vec<Unit>,
    pub messenger: Callback<Msg>
}

pub struct UnitList { }

impl Component for UnitList {
    type Message = Msg;
    type Properties = UnitListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        UnitList { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let messenger = &props.messenger;

        props.units.iter().enumerate().map(|(i, unit)| {
            html!{
                <UnitConfig
                    unit={unit.clone()}
                    id={i}
                    messenger={messenger.clone()}
                />
            }
        }).collect::<Html>()
    }
}

/*
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
*/
