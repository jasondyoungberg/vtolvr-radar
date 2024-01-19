use yew_autoprops::autoprops;
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
    AddUnit(Unit),
    DeleteUnit(usize),
    SelectUnit(usize),
}

impl Reducible for Data {
    type Action = DataAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut data = (*self).clone();
        match action {
            DataAction::AddUnit(unit) => {data.units.push(unit);}
            DataAction::DeleteUnit(index) => {data.units.remove(index);}
            DataAction::SelectUnit(index) => {data.selected = Some(index);}
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

    let data = use_reducer(|| Data {
        units: vec![
            Unit::TestUnit(TestUnit {
                name: "Test".to_string(),
                position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            }),
            Unit::F45(F45 {
                name: "Fighter".to_string(),
                position: nalgebra::Vector3::new(
                    3_000.0, 5_000.0, 15_000.0,
                ),
            }),
            Unit::Mad4(Mad4 {
                name: "Radar".to_string(),
                position: nalgebra::Vector3::new(
                    -2_000.0, 100.0, -10_000.0,
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
