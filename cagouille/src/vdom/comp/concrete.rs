use std::any::TypeId;

use futures::future::LocalBoxFuture;

use crate::component::state::AnyState;
use crate::component::traits::Component;
use crate::component::State;

use crate::df::traits::Differentiable;
use crate::vdom::traits::Renderable;
use crate::vdom::Mode;
use crate::vdom::Scope;
use crate::vdom::VNode;
use crate::vdom::VNodeKey;

use super::df::AnyComponentDf;
use super::df::AnyComponentStateDf;
use super::driver::CompDriver;
use super::ComponentNode;

pub enum ConcreteComponentNode<M, Comp: Component<M>>
where
    Comp: Component<M>,
    M: Mode,
{
    Uninitialised {
        props: Comp::Properties,
        events: Comp::Events,
    },
    Initialised {
        state: State<M, Comp>,
    },
}

impl<M, Comp> ConcreteComponentNode<M, Comp>
where
    Comp: Component<M> + 'static,
    M: Mode,
{
    pub fn new(props: Comp::Properties, events: Comp::Events) -> Self {
        Self::Uninitialised { props, events }
    }

    pub fn is_initialised(&self) -> bool {
        match self {
            Self::Initialised { state } => true,
            _ => false,
        }
    }
}

impl<M, Comp> CompDriver<M> for ConcreteComponentNode<M, Comp>
where
    Comp: Component<M> + 'static,
    M: Mode,
    Comp::Properties: Differentiable,
{
    /// Initialise the component
    fn initialise<'a, 'fut>(&'a mut self) -> LocalBoxFuture<'fut, Result<(), Error>>
    where
        'a: 'fut,
    {
        if self.is_initialised() {
            return std::future::ready(Ok(()));
        }

        Box::pin(async {
            // Initialise the state
            self.state.initialise().await;

            // Depending on the mode, we perform additional operations
            M::on_component_node_initialised(self).await;

            Ok(())
        })
    }

    fn id(&self) -> &VNodeKey {
        &self.key
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<Comp>()
    }

    fn df<'a, 'fut>(
        &'a self,
        other: &'a ComponentNode<M>,
    ) -> LocalBoxFuture<'fut, super::df::AnyComponentDf>
    where
        'a: 'fut,
    {
        Box::pin(async {
            let maybe_other_s = other.state::<Comp>();

            if maybe_other_s.is_none() {
                return AnyComponentDf::Replace;
            }

            let other_s = maybe_other_s.unwrap();
            let state_df = State::<M, Comp>::df(&self.state, &other_s).await;

            AnyComponentDf::Update(AnyComponentStateDf(Box::new(state_df)))
        })
    }

    fn state(&self) -> AnyState {
        self.state.to_any()
    }
}

impl<M, Comp> Into<ComponentNode<M>> for ConcreteComponentNode<M, Comp>
where
    Comp: Component<M> + 'static,
    M: Mode,
    Comp::Properties: Differentiable,
{
    fn into(self) -> ComponentNode<M> {
        ComponentNode {
            v_node: self.v_node.clone(),
            driver: Box::new(self),
        }
    }
}

impl<M, Comp> Into<VNode<M>> for ConcreteComponentNode<M, Comp>
where
    Comp: Component<M> + 'static,
    M: Mode,
    Comp::Properties: Differentiable,
{
    fn into(self) -> VNode<M> {
        VNode::Component(self.into())
    }
}
