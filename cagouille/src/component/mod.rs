use crate::vdom::mode::Mode;

pub mod ctx;
pub mod event;
pub mod state;

pub enum ComponentEvent<'props, M, Component> where Component: traits::Component<M>, M: Mode {
    PropertiesChanged{previous: &'props Component::Properties},
    Rendered,
    Destroyed
}

pub mod traits {

    use futures::future::LocalBoxFuture;

    use crate::{error::Error, vdom::{VNode, mode::Mode}};
    use super::ctx::Context;

    pub trait Component<M>: Sized where M: Mode {
        /// Properties of the component
        type Properties: Send + Sync + Default;
        
        /// Events
        type Events: Default;

        /// Internal data of the component
        type Data: Send + Sync;

        /// Create a new component
        fn data<'props, 'fut>(props: &'props Self::Properties) -> LocalBoxFuture<'fut, Self::Data> where 'props: 'fut;
        
        /// Render the component.
        fn render<'ctx, 'fut>(data: Context<'ctx, M, Self>) -> LocalBoxFuture<'fut, Result<VNode<M>, Error>> where 'ctx: 'fut;
    }
}