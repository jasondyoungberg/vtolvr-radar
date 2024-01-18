use crate::*;

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
