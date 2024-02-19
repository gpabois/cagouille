mod context;
mod state;

pub use state::State;

pub mod traits {
    use std::fmt::Debug;

    use crate::{df::traits::Differentiable, error::Error, vdom::VNode};

    use super::context::{Context, InitContext};

    pub trait Component: Sized {
        /// Properties of the component
        type Properties: Send + Sync + Default + Differentiable + Debug;

        /// Events
        type Events: Default + Send + Sync + Debug;

        /// Internal data of the component
        type Data: Send + Sync + Debug;

        /// Initialise component state
        fn initialise<'props, 'fut>(ctx: InitContext<Self>) -> Self::Data;

        /// Render the component.
        fn render<'ctx>(ctx: Context<'ctx, Self>) -> Result<VNode, Error>;
    }
}
