use rand::random;
use yew::prelude::*;
use yew_autoprops::autoprops;
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

#[autoprops]
#[function_component]
pub fn DataProvider(
    children: Html
) -> Html {
    use crate::unit::*;

    let data = use_reducer(|| Data {
        units: rand::random::<[Unit; 10]>().to_vec(),
        selected: None,
    });

    html! {
        <ContextProvider<DataContext> context={data}>
            {children}
        </ContextProvider<DataContext>>
    }
}
