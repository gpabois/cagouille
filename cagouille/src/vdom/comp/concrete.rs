use std::any::TypeId;

use futures::future::LocalBoxFuture;

use crate::component::state::AnyState;
use crate::component::traits::Component;
use crate::component::State;

use crate::df::traits::AsyncDifferentiable;
use crate::df::traits::Differentiable;
use crate::vdom::VNode;
use crate::vdom::Mode;
use crate::vdom::Scope;
use crate::vdom::VNodeKey;
use crate::vdom::traits::Renderable;

use super::df::AnyComponentDf;
use super::df::AnyComponentStateDf;
use super::driver::ComponentNodeDriver;
use super::AnyComponentNode;

pub struct ComponentNode<M, Comp: Component<M>> where Comp: Component<M>, M: Mode {
    pub key: VNodeKey,
    pub state: State<M, Comp>
 }

impl<M, Comp> Default for ComponentNode<M, Comp> where Comp: Component<M>, M: Mode {
    fn default() -> Self {
        Self {
            key: VNodeKey::default(),
            state: Default::default(),
        }
    }
}

impl<M, Comp> ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode {
    pub fn new(parent: &Scope, props: Comp::Properties, events: Comp::Events) -> Self {
        let scope = parent.new_child_scope();
        let node_key = scope.id.clone();
        
        Self {
            key: node_key,
            state: State::new(scope, props, events),
        }
    }
}

impl<'a, M, Comp> Renderable<'a, M> for &'a ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode{
    fn render<'fut>(self) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>> where 'a: 'fut {
        Box::pin(self.state.render())
    }
}


impl<M, Comp> ComponentNodeDriver<M> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode, Comp::Properties: Differentiable 
{    
    /// Initialise the component
    fn initialise<'a, 'fut>(&'a self) -> LocalBoxFuture<'fut, Result<(), crate::error::Error>> where 'a: 'fut {
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

    fn df<'a, 'fut>(&'a self, other: &'a AnyComponentNode<M>) -> LocalBoxFuture<'fut, super::df::AnyComponentDf> 
    where 'a: 'fut {
        Box::pin(async {
            let maybe_other_s = other.state::<Comp>();
        
            if maybe_other_s.is_none() {
                return AnyComponentDf::Replace
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

impl<M, Comp> Into<AnyComponentNode<M>> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode, Comp::Properties: Differentiable {
    fn into(self) -> AnyComponentNode<M> {
        AnyComponentNode {
            v_node: self.v_node.clone(),
            driver: Box::new(self)
        }
    }
}

impl<M, Comp> Into<VNode<M>> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode, Comp::Properties: Differentiable {
    fn into(self) -> VNode<M> {
        VNode::Component(self.into())
    }
}
