
pub mod function;


pub enum RenderMode {
    Browser,
    SSR,
    Hydration
}

pub struct State<Component> where Component: traits::Component {
    pub props: Component::Properties,
    pub mode: RenderMode,
    pub dehydrated: Option<String>,
}

pub enum ComponentEvent<'props, Component> where Component: traits::Component {
    PropertiesChanged{previous: &'props Component::Properties},
    Rendered,
    Destroyed
}

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{error::Error, vdom::VNode};
    use super::{State, ComponentEvent};

    pub trait Component: Sized {
        /// Events triggered by the component
        type Events: Send + Sync;

        /// Properties of the component
        type Properties: Send + Sync;

        /// Create a new component
        fn new(state: State<Self>) -> Self;

        /// Process an event.
        fn process_event<'state, 'fut>(&mut self, state: &'state State<Self>, event: ComponentEvent<'_, Self>)  
        -> BoxFuture<'fut, Result<(), Error>> where 'state: 'fut;

        /// Render the component.
        fn render<'s, 'fut>(&self, state: &'s State<Self>) -> BoxFuture<'fut, Result<VNode, Error>> where 's: 'fut;

    }


    pub trait Mutator<Component> 
    where Component: super::traits::Component {
        type Mutation;
        fn execute(self, mutation: Self::Mutation) -> Self;
    }

}