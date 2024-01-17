pub mod state;

pub enum ComponentEvent<'props, Component> where Component: traits::Component {
    PropertiesChanged{previous: &'props Component::Properties},
    Rendered,
    Destroyed
}

pub mod traits {
    use futures::future::BoxFuture;

    use crate::{error::Error, vdom::VNode};
    use super::{ComponentEvent, state::State};

    pub trait Component: Sized {
        /// Properties of the component
        type Properties: Send + Sync;
        
        /// Internal data of the component
        type Data: Send + Sync;

        /// Create a new component
        fn data<'props, 'fut>(props: &'props Self::Properties) -> BoxFuture<'fut, Self::Data> where 'props: 'fut;
        
        /// Render the component.
        fn render<'s, 'fut>(state: &'s State<Self>) -> BoxFuture<'fut, Result<VNode, Error>> where 's: 'fut;
    }

}