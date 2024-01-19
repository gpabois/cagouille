
use std::sync::Arc;

use async_std::sync::RwLock;
use futures::{future::LocalBoxFuture, AsyncWrite};

use crate::component::{traits::Component, state::State};
use super::{Scope, traits::{Renderable, RenderToStream}, VNode, mode::Mode};

pub trait IComponentNode<M> where M: Mode {
    /// Returns the component's root virtual node.
    fn get_node<'fut>(&self) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>>;
}

pub struct AnyComponentNode<M>(Box<dyn IComponentNode<M>>);

impl<'a, M> RenderToStream<'a> for &'a AnyComponentNode<M> {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + futures::AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        todo!()
    }
}

struct Inner<M: Mode> {
    v_node: Option<VNode<M>>,
    mode_state: M::ComponentNodeState
}

impl<M: Mode> Inner<M> where {
    pub fn new() -> Self {
        Self { v_node: None, mode_state: Default::default() }
    }
}

pub struct ComponentNode<M, Comp: Component<M>> where Comp: Component<M>, M: Mode {
    inner: Arc<RwLock<Inner<M>>>,
    state: State<M, Comp>
 }

impl<M, Comp> Default for ComponentNode<M, Comp> where Comp: Component<M>, M: Mode {
    fn default() -> Self {
        Self {
            state: Default::default(),
            inner: Arc::new(RwLock::new(Inner::new()))
        }
    }
}

impl<M, Comp> ComponentNode<M, Comp> where Comp: Component<M>, M: Mode {
    pub fn new(parent: &Scope, props: Comp::Properties, events: Comp::Events) -> Self {
        Self {
            state: State::new(parent.new_child_scope(), props, events),
            inner: Arc::new(RwLock::new(Inner::new()))
        }
    }

    /// Initialise the node, if it has not been yet initialised.
    pub async fn initialise(&self) {
        self
        .state
        .initialise()
        .await;
    }
}

impl<'a, M, Comp> Renderable<'a, M> for &'a ComponentNode<M, Comp> where Comp: Component<M>, M: Mode{
    fn render<'fut>(self) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>> where 'a: 'fut {
        Box::pin(self.state.render())
    }
}


impl<M, Comp> IComponentNode<M> for ComponentNode<M, Comp> where Comp: Component<M>, M: Mode {
    fn get_node<'fut>(&self) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>> {
        todo!()
    }
}

impl<M, Comp> Into<AnyComponentNode<M>> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode {
    fn into(self) -> AnyComponentNode<M> {
        AnyComponentNode(Box::new(self))
    }
}

impl<M, Comp> Into<VNode<M>> for ComponentNode<M, Comp> where Comp: Component<M> + 'static, M: Mode {
    fn into(self) -> VNode<M> {
        VNode::Component(self.into())
    }
}