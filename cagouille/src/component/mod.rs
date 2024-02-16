mod context;
mod state;

pub use state::State;

pub mod traits {
    use crate::{
        df::traits::Differentiable,
        error::Error,
        vdom:: VNode,
    };

    use super::context::{Context, InitContext};

    pub trait Component: Sized {
        /// Properties of the component
        type Properties: Send + Sync + Default + Differentiable;

        /// Events
        type Events: Default + Send + Sync;

        /// Internal data of the component
        type Data: Send + Sync;

        /// Initialise component state
        fn initialise<'props, 'fut>(ctx: InitContext<Self>) -> Self::Data;

        /// Render the component.
        fn render<'ctx>(ctx: Context<'ctx, Self>) -> Result<VNode, Error>;
    }
}
