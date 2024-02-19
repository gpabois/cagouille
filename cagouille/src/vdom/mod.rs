use crate::component::traits::Component;

use self::{
    comp::ComponentNode,
    el::{attr::ElementAttribute, ElementNode},
    traits::RenderToStream,
};
use futures::{future::LocalBoxFuture, io::AsyncWriteExt, AsyncWrite};

pub mod df;
pub mod scope;

mod attr;
mod comp;
mod el;
mod mount;
mod node_key;
mod node_ref;
pub use node_key::VNodeKey;
pub use scope::Scope;
use web_sys::HtmlElement;

pub mod traits {
    use futures::future::LocalBoxFuture;
    use futures::io::{AllowStdIo, Error};
    use futures::{AsyncWrite, AsyncWriteExt};

    use std::io::{BufWriter, Cursor};

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
    Empty,
}

impl VNodeData {
    pub async fn initialise(&mut self) {
        match self {
            VNodeData::Component(comp) => comp.initialise().await,
            VNodeData::Element(el) => el.initialise().await,
            _ => {}
        }
    }
}

pub struct VNode {
    data: VNodeData,
    scope: Scope,
}

impl Default for VNode {
    fn default() -> Self {
        Self::empty(Scope::default())
    }
}

impl VNode {
    /// Creates an empty virtual node
    pub fn empty(scope: Scope) -> Self {
        Self {
            data: VNodeData::Empty,
            scope,
        }
    }

    pub fn text<IntoText>(scope: Scope, text: IntoText) -> Self
    where
        IntoText: Into<String>,
    {
        Self {
            data: VNodeData::Text(text.into()),
            scope,
        }
    }

    pub fn element<IntoTag, IntoAttrs, IntoChildren>(
        scope: Scope,
        tag: IntoTag,
        attrs: IntoAttrs,
        children: IntoChildren,
    ) -> Self
    where
        IntoTag: Into<String>,
        IntoAttrs: IntoIterator<Item = ElementAttribute>,
        IntoChildren: IntoIterator<Item = VNode>,
    {
        Self {
            scope,
            data: VNodeData::Element(ElementNode::new(tag, attrs, children)),
        }
    }

    pub fn component<Comp>(scope: Scope, props: Comp::Properties, events: Comp::Events) -> Self
    where
        Comp: Component + 'static,
    {
        Self {
            scope,
            data: VNodeData::Component(ComponentNode::new::<Comp>(props, events)),
        }
    }
}

impl VNode {
    /// Returns the vnode's key
    pub fn key(&self) -> &VNodeKey {
        &self.scope.key
    }

    /// Initialise the virtual tree
    pub async fn initialise(&mut self) {
        self.data.initialise().await
    }
}

impl<'a> RenderToStream<'a> for &'a VNodeData {
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
                Self::Component(comp) => comp.render_to_stream(stream).await?,
                Self::Element(el) => el.render_to_stream(stream).await?,
                Self::Text(text) => stream.write_all(text.as_bytes()).await?,
                Self::Empty => {}
            }

            Ok(())
        })
    }
}
