use yew::prelude::*;
use std::rc::Rc;
use crate::unit::Unit;

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub units: Vec<Unit>,
    pub selected: Option<usize>,
}

pub type DataContext = UseReducerHandle<Data>;

pub enum DataAction {
    Add(Unit),
    Delete(usize),
    Select(usize),
}

impl Reducible for Data {
    type Action = DataAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut data = (*self).clone();
        match action {
            DataAction::Add(unit) => {data.units.push(unit);}
            DataAction::Delete(index) => {data.units.remove(index);}
            DataAction::Select(index) => {data.selected = Some(index);}
        }
        Rc::new(data)
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct DataProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn DataProvider(props: &DataProviderProps) -> Html {
    use crate::unit::*;
    use crate::convert;

    let data = use_reducer(|| Data {
        units: vec![
            Unit::F45(F45 {
                name: "Fighter 1".to_string(),
                position: nalgebra::Vector3::new(
                    convert::nmi_to_m(10.0),
                    convert::ft_to_m(30_000.0),
                    convert::nmi_to_m(15.0),
                ),
            }),
            Unit::F45(F45 {
                name: "Fighter 2".to_string(),
                position: nalgebra::Vector3::new(
                    convert::nmi_to_m(-15.0),
                    convert::ft_to_m(5_000.0),
                    convert::nmi_to_m(10.0),
                ),
            }),
            Unit::Mad4(Mad4 {
                name: "Radar 1".to_string(),
                position: nalgebra::Vector3::new(
                    convert::nmi_to_m(-5.0),
                    convert::ft_to_m(1_000.0),
                    convert::nmi_to_m(-10.0),
                ),
            }),
            Unit::Mad4(Mad4 {
                name: "Radar 2".to_string(),
                position: nalgebra::Vector3::new(
                    convert::nmi_to_m(0.0),
                    convert::ft_to_m(5_000.0),
                    convert::nmi_to_m(-8.0),
                ),
            }),
            Unit::Mad4(Mad4 {
                name: "Radar 3".to_string(),
                position: nalgebra::Vector3::new(
                    convert::nmi_to_m(5.0),
                    convert::ft_to_m(2_000.0),
                    convert::nmi_to_m(-10.0),
                ),
            }),
        ],
        selected: None,
    });

    html! {
        <ContextProvider<DataContext> context={data}>
            {props.children.clone()}
        </ContextProvider<DataContext>>
    }
}
