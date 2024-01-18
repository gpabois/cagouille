pub mod ctx;
pub mod event;


pub enum ComponentEvent<'props, Component> where Component: traits::Component {
    PropertiesChanged{previous: &'props Component::Properties},
    Rendered,
    Destroyed
}


pub mod traits {

    use futures::future::LocalBoxFuture;

    use crate::{error::Error, vdom::VNode};
    use super::ctx::Context;

    pub trait Component: Sized {
        /// Properties of the component
        type Properties: Send + Sync + Default;
        
        /// Events
        type Events: Default;

        /// Internal data of the component
        type Data: Send + Sync;

        /// Create a new component
        fn data<'props, 'fut>(props: &'props Self::Properties) -> LocalBoxFuture<'fut, Self::Data> where 'props: 'fut;
        
        /// Render the component.
        fn render<'ctx, 'fut>(data: Context<'ctx, Self>) -> LocalBoxFuture<'fut, Result<VNode, Error>> where 'ctx: 'fut;
    }
}