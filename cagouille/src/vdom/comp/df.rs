use std::any::Any;

use futures::future::LocalBoxFuture;

use crate::component::traits::Component;
use crate::df::traits::{AsyncDifferentiable, Differentiable};
use crate::vdom::Mode;

use super::ComponentNode;

pub enum AnyComponentDf {
    /// Replace the whole node
    Replace,

    /// Update the component's state
    Update(AnyComponentStateDf),
}

/// Holds the concrete's state df.
pub struct AnyComponentStateDf(pub(super) Box<dyn Any>);

impl AnyComponentStateDf {
    fn downcast<Comp>(self) -> Option<StateDf<Comp>>
    where
        Comp: Component + 'static,
        Comp::Properties: Differentiable,
    {
        self.0.downcast::<StateDf<Comp>>().ok().map(|b| *b)
    }
}

impl<M: Mode> AsyncDifferentiable for ComponentNode {
    type Df = AnyComponentDf;

    fn df<'a, 'fut>(
        src: &'a ComponentNode,
        dest: &'a ComponentNode,
    ) -> LocalBoxFuture<'fut, AnyComponentDf>
    where
        'a: 'fut,
    {
        Box::pin(async {
            if src.type_id() != dest.type_id() {
                return AnyComponentDf::Replace;
            }

            src.driver.df(dest).await
        })
    }
}

