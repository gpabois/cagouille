
use std::{sync::Arc, any::TypeId};

use async_std::sync::RwLock;
use futures::{future::LocalBoxFuture, AsyncWrite};

use crate::component::{traits::Component, state::State};
use super::{Scope, traits::{Renderable, RenderToStream}, VNode, mode::Mode, node_key::VNodeKey};

pub trait IComponentNode<M> where M: Mode {
    /// Get key of the node.
    fn id(&self) -> &VNodeKey;

    /// Underlying component's type id.
    fn type_id(&self) -> TypeId;

    /// Initialise the component
    fn initialise<'a, 'fut>(&'a self) -> LocalBoxFuture<'fut, Result<(), crate::error::Error>> where 'a: 'fut;
}


pub(super) struct VNodeRef<M: Mode>(pub(super) Arc<RwLock<Option<VNode<M>>>>);

impl<M: Mode> Clone for VNodeRef<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M: Mode> Default for VNodeRef<M> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct AnyComponentNode<M: Mode> {
    /// The root node of the component
    v_node: VNodeRef<M>,

    /// Interface to the underlying component state
    driver: Box<dyn IComponentNode<M>>
}

impl<M: Mode> AnyComponentNode<M> {
    pub async fn initialise(&self) {
        self
        .driver
        .initialise()
        .await
        .expect("cannot initialise component");
    }

    pub fn id(&self) -> &VNodeKey {
        self.driver.id()
    }

    pub fn type_id(&self) -> TypeId {
        self.driver.type_id()
    }
}

impl<'a, M> RenderToStream<'a> for &'a AnyComponentNode<M> where M: Mode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + futures::AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async {
            self.v_node.0.read().await.as_ref()
            .expect("vnode has not been rendered")
            .render_to_stream(stream)
            .await
        })
    }
}


pub struct ComponentNode<M, Comp: Component<M>> where Comp: Component<M>, M: Mode {
    pub(super) key: VNodeKey,
    pub(super) v_node: VNodeRef<M>,
    pub(super) state: State<M, Comp>
 }

impl<M, Comp> Default for ComponentNode<M, Comp> where Comp: Component<M>, M: Mode {
    fn default() -> Self {
        Self {
            key: VNodeKey::default(),
            state: Default::default(),
            v_node: Default::default()
        }
    }
}

impl<M, Comp> ComponentNode<M, Comp> where Comp: Component<M>, M: Mode {
    pub fn new(parent: &Scope, props: Comp::Properties, events: Comp::Events) -> Self {
        let scope = parent.new_child_scope();
        let node_key = scope.id.clone();
        Self {
            key: node_key,
            state: State::new(scope, props, events),
            v_node: Default::default()
        }
    }
}

impl<'a, M, Comp> Renderable<'a, M> for &'a ComponentNode<M, Comp> where Comp: Component<M>, M: Mode{
    fn render<'fut>(self) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>> where 'a: 'fut {
        Box::pin(self.state.render())
    }
}


impl<M, Comp> IComponentNode<M> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode 
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
}

impl<M, Comp> Into<AnyComponentNode<M>> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode {
    fn into(self) -> AnyComponentNode<M> {
        AnyComponentNode {
            v_node: self.v_node.clone(),
            driver: Box::new(self)
        }
    }
}

impl<M, Comp> Into<VNode<M>> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode {
    fn into(self) -> VNode<M> {
        VNode::Component(self.into())
    }
}