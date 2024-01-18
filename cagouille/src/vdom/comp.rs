use std::{any::TypeId, rc::Rc};

use async_std::sync::RwLock;
use futures::{future::LocalBoxFuture, AsyncWrite, Future};
use web_sys::Node;

use crate::component::{traits::Component, ctx::Context, event::{traits::{EventSignal, Event}, EventSlot}};
use super::{VNodeScope, traits::{Renderable, RenderToStream}, VNode};

pub trait IComponentNode {
    /// Return the component's root virtual node.
    fn get_node<'a, 'fut>(&'a self) -> LocalBoxFuture<'fut, Result<ComponentVNodeRef, crate::error::Error>> where 'a: 'fut;
    
    /// Type ID of the component
    fn type_id(&self) -> TypeId;
}

pub struct AnyComponentNode(Box<dyn IComponentNode>);

impl<'a> RenderToStream<'a> for &'a AnyComponentNode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + futures::AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async {
            self.0.get_node()
        })
    }
}
struct Inner<Comp: Component> {
    // scope of the node
    scope: VNodeScope,

    // properties of the node
    props: Comp::Properties,

    // events
    events: Comp::Events,

    // state
    state: Option<Comp::Data>,

    // last render node
    v_node: Option<VNode>,

    // mounted root node
    dom_node: Option<Node>
}

impl<Comp> Default for Inner<Comp> where Comp: Component {
    fn default() -> Self {
        Self { scope: Default::default(), props: Default::default(), events: Default::default(), state: Default::default(), v_node: Default::default(), dom_node: Default::default() }
    }
}

#[derive(Clone)]
pub struct ComponentNodeState<Comp>(Rc<RwLock<Inner<Comp>>>) where Comp: Component;

pub struct ComponentNode<Comp>(ComponentNodeState<Comp>) where Comp: Component;

impl<Comp> Default for ComponentNode<Comp> where Comp: Component {
    fn default() -> Self {
        Self(Rc::new(RwLock::new(Default::default())))
    }
}

impl<Comp> ComponentNode<Comp> where Comp: Component {
    pub fn new(parent: &VNodeScope, props: Comp::Properties) -> Self {
        Self {
            scope: parent.new_child_scope(),
            props,
            events: Default::default(),
            state: Default::default(),
            root: Default::default()
        }
    }

    /// Consume the mutable reference, replace its content with default value, and returns the value
    pub fn consume(&mut self) -> Self {
        std::mem::replace(self, Self::default())
    }  

    /// Initialise the component state if it has not been initialised yet.
    async fn initialise_if_not(&self) {
        if self.state.read().await.is_none() {
            let data = Comp::data(&self.props).await;
            *self.state.write().await = Some(data);
        }
    }

    pub fn on<'a, E: Event>(&mut self, _: E, slot: EventSlot<'a, E>) -> &mut Self where Comp::Events: EventSignal<'a, E> {
        self.events.connect(slot);
        self
    }
}

impl<'a, Comp> Renderable<'a> for &'a ComponentNode<Comp> where Comp: Component {
    fn render<'fut>(self) -> LocalBoxFuture<'fut, Result<VNode, crate::error::Error>> where 'a: 'fut {
        Box::pin(async {
            self.initialise_if_not().await;

            let borrowed_data = self.state.read().await;
            let data = borrowed_data.as_ref().expect("not initialised");

            let ctx = Context{data, scope: &self.scope};
            Comp::render(ctx).await
        })
    }
}

impl<Comp> IComponentNode for ComponentNode<Comp> where Comp: Component + 'static {
    fn get_node<'a, 'fut>(&'a self) -> LocalBoxFuture<'fut, Result<ComponentVNodeRef, crate::error::Error>>
    where 'a: 'fut
    {
        Box::pin(async {
            if !self.root.is_rendered().await {
                let root = self.render().await?;
                self.root.set(root).await;
            }
    
            Ok(self.root.clone())
        })
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<Comp>()
    }
}

impl<Comp> Into<AnyComponentNode> for ComponentNode<Comp> where Comp: Component + 'static {
    fn into(self) -> AnyComponentNode {
        AnyComponentNode(Box::new(self))
    }
}

impl<Comp> Into<VNode> for ComponentNode<Comp> where Comp: Component + 'static {
    fn into(self) -> VNode {
        VNode::Component(self.into())
    }
}