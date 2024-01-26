
use std::{fmt::Debug, sync::Arc};

use crate::component::traits::Component;

use self::{traits::RenderToStream, el::ElementNode, comp::ComponentNode, mode::Mode};
use async_std::sync::RwLock;
use futures::{io::AsyncWriteExt, future::{LocalBoxFuture, join_all}, AsyncWrite};
use seeded_random::{Seed, Random};

pub mod mode;
pub mod df;
pub mod scope;

mod attr;
mod el;
mod comp;
mod node_ref;
mod node_key;

pub use scope::Scope;
pub use node_key::VNodeKey;

pub mod traits {
    use futures::{AsyncWrite, AsyncWriteExt};
    use futures::io::{Error, AllowStdIo};
    use futures::future::LocalBoxFuture;
    
    use std::io::{BufWriter, Cursor};

    use super::VNode;
    use super::mode::Mode;

    pub trait Renderable<'a, M>: Sized + 'a where M: Mode {
        /// Render the object and returns a virtual dom's node.
        fn render<'fut>(self) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>> where 'a: 'fut;
    }

    /// Render object in the stream
    pub trait RenderToStream<'a>: Sized + 'a {
        /// Render the virtual to a stream.
        fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
        -> LocalBoxFuture<'fut, Result<(), Error>>
        where 'a: 'fut, 'stream: 'fut;

        /// Render the virtual dom to a string.
        fn render_to_string<'fut>(self) -> LocalBoxFuture<'fut, Result<String, Error>> where 'a: 'fut {
            Box::pin(async {
                let mut output = Vec::<u8>::default();

                // Scoping to release the mut ref to output.
                {
                    let mut stream = AllowStdIo::new(
                        BufWriter::new(
                            Cursor::new(&mut output)
                        )
                    );
    
                    self.render_to_stream(&mut stream).await?;
                }


                Ok(String::from_utf8(output).unwrap()) 
            })
        }
    }
}


/// A sharable, and mutable reference to a node.
pub(self) struct SharedVNode<M: Mode>(pub(super) Arc<RwLock<Option<VNode<M>>>>);

impl<M: Mode> Clone for SharedVNode<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M: Mode> Default for SharedVNode<M> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct MaybeVNodeRef<'a, M>(Option<&'a VNode<M>>) where M: Mode;

pub enum VNode<M> where M: Mode {
    Component(comp::AnyComponentNode<M>),
    Element(el::ElementNode<M>),
    Text(String),
    Empty
}

impl<M> Debug for VNode<M> where M: Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Component(_) => f.debug_tuple("Component").finish(),
            Self::Element(_) => f.debug_tuple("Element").finish(),
            Self::Text(arg0) => f.debug_tuple("Text").field(arg0).finish(),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

impl<M> VNode<M> where M: Mode {
    pub fn id(&self) -> Option<&VNodeKey> {
        match self {
            VNode::Component(comp) => Some(comp.id()),
            VNode::Element(el) => Some(el.id()),
            VNode::Text(_) => None,
            VNode::Empty => None,
        }
    }

    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn text<IntoStr: Into<String>>(text: IntoStr) -> Self {
        Self::Text(text.into())
    }

    pub fn element<IntoStr: Into<String>>(parent: &Scope, tag: IntoStr) -> ElementNode<M> {
        ElementNode::new(parent, tag)
    }

    pub fn component<Comp>(parent: &Scope, props: Comp::Properties, events: Comp::Events) -> ComponentNode<M, Comp> where Comp: Component<M> + 'static {
        ComponentNode::new(parent, props, events)
    }

    pub fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a VNode<M>> + 'a> {
        match self {
            VNode::Element(el) => Box::new(el.iter_children()),
            _ => Box::new(std::iter::empty())
        }
    }
    
    /// Initialise the virtual tree
    pub fn initialise<'a, 'fut>(&'a self) -> LocalBoxFuture<'fut, ()> 
    where 'a: 'fut 
    {
        Box::pin(async move {
            match self {
                VNode::Component(comp) => {
                    comp.initialise().await;
                },
                VNode::Element(el) => {
                    join_all(el.iter_children().map(|n| {
                        n.initialise()
                    })).await;
                },
                VNode::Text(_) => {},
                VNode::Empty => {},
            }
        })
    }
}

impl<'a, M> RenderToStream<'a> for &'a VNode<M> where M: Mode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async move {
            match self {
                VNode::Component(comp) => comp.render_to_stream(stream).await?,
                VNode::Element(el) => el.render_to_stream(stream).await?,
                VNode::Text(text) =>  stream.write_all(text.as_bytes()).await?,
                VNode::Empty => {},
            }

            Ok(())
        })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::prelude::Self_Differentiable;
    use futures::future::LocalBoxFuture;

    use crate::{
        component::{traits::Component, ctx::Context}, 
        vdom::mode::DebugMode, 
        event::{EventSlot, traits::{EventSignal, Event}}
    };

    use super::{VNode, mode::Mode};

    pub struct BarData{attr: String}

    #[derive(Default, Self_Differentiable)]
    pub struct BarProps{attr: String}

    pub struct BarChanged;
    impl Event for BarChanged 
    {
        type Payload = usize;
    }

    #[derive(Default)]
    pub struct BarEvents<'a> {
        changed: Option<EventSlot<'a, BarChanged>>
    }

    impl<'a> EventSignal<'a, BarChanged> for BarEvents<'a> {
        fn connect(&mut self, slot: EventSlot<'a, BarChanged>) {
            self.changed = Some(slot)
        }

        fn emit(&self, payload: <BarChanged as Event>::Payload) {
            if let Some(slot) = &self.changed {
                slot.received(payload)
            }
        }
    }

    pub struct Bar<'a>{_marker: std::marker::PhantomData<&'a()>}

    impl<'a, M> Component<M> for Bar<'a> where M: Mode {
        type Properties = BarProps;
        type Data = BarData;
        type Events = BarEvents<'a>;

        fn data<'props, 'fut>(props: &'props Self::Properties) -> LocalBoxFuture<'fut, Self::Data> where 'props: 'fut {
            Box::pin(std::future::ready(Self::Data{
                attr: props.attr.clone()
            }))
        }

        
        fn render<'s, 'fut>(ctx: Context<'s, M, Self>) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>> where 's: 'fut {
            Box::pin(async move {
                VNode::element(&ctx.scope, "div")
                .append_child(VNode::text(&ctx.data.attr))
                .consume()
                .into()
            })
        }

        fn initialised<'ctx, 'fut>(ctx: crate::component::ctx::MutContext<'ctx, M, Self>) -> LocalBoxFuture<'fut, ()> where 'ctx: 'fut {
            Box::pin(async {})
        }
    }

    pub struct FooData{}

    #[derive(Default, Self_Differentiable)]
    pub struct FooProps{}
    pub struct Foo;

    impl<M> Component<M> for Foo where M: Mode {
        type Properties = FooProps;
        type Data = FooData;
        type Events = ();

        fn data<'props, 'fut>(props: &'props Self::Properties) -> LocalBoxFuture<'fut, Self::Data> where 'props: 'fut {
            Box::pin(std::future::ready(Self::Data{}))
        }

        fn render<'s, 'fut>(ctx: Context<'s, M, Self>) -> LocalBoxFuture<'fut, Result<super::VNode<M>, crate::error::Error>> where 's: 'fut {
            Box::pin(async move {
                VNode::element(&ctx.scope, "div")
                .extend_child((0..10_000).map(|_| 
                    VNode::component::<Bar>(
                        &ctx.scope, 
                        BarProps{
                            attr: "Hello world".into(),
                            ..Default::default()
                        },
                        BarEvents{
                            ..Default::default()
                        }
                    )
                    .into()
                ))
                .consume()
                .into()
            })      
        }

        fn initialised<'ctx, 'fut>(ctx: crate::component::ctx::MutContext<'ctx, M, Self>) -> LocalBoxFuture<'fut, ()> where 'ctx: 'fut {
            todo!()
        }
    }

    #[tokio::test]
    pub async fn test_foo_root() {
        let tree = DebugMode::render_to_string::<Foo>(FooProps{}, ()).await;
        println!("{}", tree)

    }
}

