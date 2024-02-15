use std::any::Any;

use futures::future::LocalBoxFuture;

use crate::component::state::StateDf;
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
    fn downcast<Comp: Component<M> + 'static, M: Mode>(self) -> Option<StateDf<M, Comp>>
    where
        Comp::Properties: Differentiable,
    {
        self.0.downcast::<StateDf<M, Comp>>().ok().map(|b| *b)
    }
}

impl<M: Mode> AsyncDifferentiable for ComponentNode<M> {
    type Df = AnyComponentDf;

    fn df<'a, 'fut>(
        src: &'a ComponentNode<M>,
        dest: &'a ComponentNode<M>,
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

