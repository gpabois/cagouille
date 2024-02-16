use std::{fmt::Debug, sync::Arc};

use crate::component::traits::Component;

use self::{comp::ConcreteComponentNode, el::ElementNode, mode::Mode, traits::RenderToStream};
use async_std::sync::RwLock;
use futures::{
    future::{join_all, LocalBoxFuture},
    io::AsyncWriteExt,
    AsyncWrite,
};

pub mod df;
pub mod mode;
pub mod scope;

mod attr;
mod comp;
mod el;
mod node_key;
mod node_ref;

pub use node_key::VNodeKey;
pub use scope::Scope;

pub mod traits {
    use futures::future::LocalBoxFuture;
    use futures::io::{AllowStdIo, Error};
    use futures::{AsyncWrite, AsyncWriteExt};

    use std::io::{BufWriter, Cursor};

    use super::mode::Mode;
    use super::VNode;

    pub trait Renderable<'a, M>: Sized + 'a
    where
        M: Mode,
    {
        /// Render the object and returns a virtual dom's node.
        fn render<'fut>(self) -> LocalBoxFuture<'fut, Result<VNode<M>, crate::error::Error>>
        where
            'a: 'fut;
    }

    /// Render object in the stream
    pub trait RenderToStream<'a>: Sized + 'a {
        /// Render the virtual to a stream.
        fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(
            self,
            stream: &'stream mut W,
        ) -> LocalBoxFuture<'fut, Result<(), Error>>
        where
            'a: 'fut,
            'stream: 'fut;

        /// Render the virtual dom to a string.
        fn render_to_string<'fut>(self) -> LocalBoxFuture<'fut, Result<String, Error>>
        where
            'a: 'fut,
        {
            Box::pin(async {
                let mut output = Vec::<u8>::default();

                // Scoping to release the mut ref to output.
                {
                    let mut stream = AllowStdIo::new(BufWriter::new(Cursor::new(&mut output)));

                    self.render_to_stream(&mut stream).await?;
                }

                Ok(String::from_utf8(output).unwrap())
            })
        }
    }
}

enum VNodeData {
    Component(comp::ComponentNode),
    Element(el::ElementNode),
    Text(String),
    Empty
}
pub struct VNode
{
    data: VNodeData,
    key: VNodeKey
}

impl<M> Debug for VNode<M>
where
    M: Mode,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Component(_) => f.debug_tuple("Component").finish(),
            Self::Element(_) => f.debug_tuple("Element").finish(),
            Self::Text(arg0) => f.debug_tuple("Text").field(arg0).finish(),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

impl<M> VNode<M>
where
    M: Mode,
{
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

    pub fn component<Comp>(
        parent: &Scope,
        props: Comp::Properties,
        events: Comp::Events,
    ) -> ConcreteComponentNode<M, Comp>
    where
        Comp: Component<M> + 'static,
    {
        ConcreteComponentNode::new(parent, props, events)
    }

    pub fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a VNode<M>> + 'a> {
        match self {
            VNode::Element(el) => Box::new(el.iter_children()),
            _ => Box::new(std::iter::empty()),
        }
    }

    /// Initialise the virtual tree
    pub fn initialise<'a, 'fut>(&'a self) -> LocalBoxFuture<'fut, ()>
    where
        'a: 'fut,
    {
        Box::pin(async move {
            match self {
                VNode::Component(comp) => {
                    comp.initialise().await;
                }
                VNode::Element(el) => {
                    join_all(el.iter_children().map(|n| n.initialise())).await;
                }
                VNode::Text(_) => {}
                VNode::Empty => {}
            }
        })
    }
}

impl<'a, M> RenderToStream<'a> for &'a VNode<M>
where
    M: Mode,
{
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(
        self,
        stream: &'stream mut W,
    ) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where
        'a: 'fut,
        'stream: 'fut,
    {
        Box::pin(async move {
            match self {
                VNode::Component(comp) => comp.render_to_stream(stream).await?,
                VNode::Element(el) => el.render_to_stream(stream).await?,
                VNode::Text(text) => stream.write_all(text.as_bytes()).await?,
                VNode::Empty => {}
            }

            Ok(())
        })
    }
}
