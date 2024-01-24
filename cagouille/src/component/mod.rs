use crate::vdom::mode::Mode;

pub mod ctx;
pub mod event;
pub mod state;

pub use state::State;

pub enum ComponentEvent<'props, M, Component> where Component: traits::Component<M>, M: Mode {
    PropertiesChanged{previous: &'props Component::Properties},
    Rendered,
    Destroyed
}

pub mod traits {

    use futures::future::LocalBoxFuture;

    use crate::{df::traits::Differentiable, error::Error, vdom::{VNode, mode::Mode}};
    use super::ctx::{Context, MutContext};

    pub trait Component<M>: Sized where M: Mode {
        /// Properties of the component
        type Properties: Send + Sync + Default + Differentiable;
        
        /// Events
        type Events: Default;

        /// Internal data of the component
        type Data: Send + Sync;

        /// Create a new component
        fn data<'props, 'fut>(props: &'props Self::Properties) -> LocalBoxFuture<'fut, Self::Data> where 'props: 'fut;
        
        /// Called after the component has been initialised
        fn initialised<'ctx, 'fut>(ctx: MutContext<'ctx, M, Self>) -> LocalBoxFuture<'fut, ()> where 'ctx: 'fut;

        /// Render the component.
        fn render<'ctx, 'fut>(ctx: Context<'ctx, M, Self>) -> LocalBoxFuture<'fut, Result<VNode<M>, Error>> where 'ctx: 'fut;
    }
}