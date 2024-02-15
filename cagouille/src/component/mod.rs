mod context;
mod state;

pub mod traits {

    use futures::future::LocalBoxFuture;

    use crate::{
        df::traits::Differentiable,
        error::Error,
        vdom::{mode::Mode, VNode},
    };

    use super::context::{Context, InitContext};

    pub trait Component<M>: Sized
    where
        M: Mode,
    {
        /// Properties of the component
        type Properties: Send + Sync + Default + Differentiable;

        /// Events
        type Events: Default + Send + Sync;

        /// Internal data of the component
        type Data: Send + Sync;

        /// Initialise component state
        fn initialise<'props, 'fut>(ctx: InitContext<M, Self>) -> Self::Data;

        /// Render the component.
        fn render<'ctx>(ctx: Context<'ctx, M, Self>) -> Result<VNode<M>, Error>;
    }
}
