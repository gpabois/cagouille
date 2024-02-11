mod state;
mod context;

pub mod traits {

    use futures::future::LocalBoxFuture;

    use crate::{df::traits::Differentiable, error::Error, vdom::{VNode, mode::Mode}};

    use super::context::{Context, InitContext};

    pub trait Component<M>: Sized where M: Mode {
        /// Properties of the component
        type Properties: Send + Sync + Default + Differentiable;
        
        /// Events
        type Events: Default;

        /// Internal data of the component
        type Data: Send + Sync;

        /// Initialise component state
        fn initialise<'props, 'fut>(ctx: InitContext<M, Self>) -> Self::Data;
        
        /// Render the component.
        fn render<'ctx>(ctx: Context<'ctx, M, Self>) -> Result<VNode<M>, Error>;
    }
}
