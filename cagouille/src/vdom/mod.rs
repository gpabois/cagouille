
use crate::{error::Error, component::traits::Component};

use self::{traits::RenderToStream, node_key::VNodeKey, el::{ElementAttributes, ElementNode}, comp::ComponentNode};
use futures::{io::AsyncWriteExt, future::LocalBoxFuture, AsyncWrite};
use seeded_random::{Seed, Random};

mod attr;
mod el;
mod comp;
mod node_ref;
mod node_key;

pub type VDomResult = Result<VNode, Error>;

#[derive(Clone)]
pub enum RenderMode {
    Browser,
    SSR,
    Hydration
}

/// Node's scope
pub struct VDomNodeScope {
    /// Node key
    pub id: node_key::VNodeKey,
    
    /// Render mode
    pub mode: RenderMode, 
    
    /// The rng generator for children node keys.
    pub(self) rng: Random
}

impl Clone for VDomNodeScope {
    fn clone(&self) -> Self {
        Self::new(self.id.0, self.mode.clone())
    }
}

impl VDomNodeScope {
    /// New root scope
    pub fn new_root(mode: RenderMode) -> Self {
        Self { id: Default::default(), mode, rng: Random::from_seed(Seed::unsafe_new(0)) }
    }
    
    /// Create a new rendering scope
    pub fn new(id: u32, mode: RenderMode) -> Self {
        Self { 
            id: id.into(), 
            mode: mode,
            rng: Random::from_seed(Seed::unsafe_new(id.into()))  
        }
    }

    /// Create a child scope
    pub fn new_child_scope(&self) -> VDomNodeScope {
        Self::new(self.rng.u32(), self.mode.clone())
    }
}

pub mod traits {

    use futures::{AsyncWrite, AsyncWriteExt};
    use futures::io::{Error, AllowStdIo};
    use futures::future::LocalBoxFuture;
    
    use std::io::{BufWriter, Cursor};

    use super::VNode;

    pub trait Renderable<'a>: Sized + 'a {
        /// Render the object and returns a virtual dom's node.
        fn render<'fut>(self) -> LocalBoxFuture<'fut, Result<VNode, crate::error::Error>> where 'a: 'fut;
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

#[derive(Clone)]
/// Virtual DOM node
pub enum VNode {
    Element(el::ElementNode),
    Text(String),
    Empty
}

impl VNode {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn text<IntoStr: Into<String>>(text: IntoStr) -> Self {
        Self::Text(text.into())
    }

    pub fn element<IntoStr: Into<String>>(parent: &VDomNodeScope, tag: IntoStr) -> ElementNode {
        ElementNode::new(parent, tag)
    }

    pub async fn component<Comp: Component>(parent: &VDomNodeScope, props: Comp::Properties) -> ComponentNode<Comp> {
        ComponentNode::new(parent, props).await
    }
}

impl<'a> RenderToStream<'a> for &'a VNode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async move {
            match self {
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
    use crate::{component::{traits::Component, state::State}, vdom::{VDomNodeScope, RenderMode, traits::{Renderable, RenderToStream}}};

    use super::VNode;

    pub struct FooData{
        attr: String
    }
    pub struct FooProps{}
    
    pub struct Foo;

    impl Component for Foo {
        type Properties = FooProps;
        type Data = FooData;

        fn data<'props, 'fut>(props: &'props Self::Properties) -> futures::prelude::future::BoxFuture<'fut, Self::Data> where 'props: 'fut {
            Box::pin(std::future::ready(Self::Data{attr: "hello world !".into()}))
        }

        fn render<'s, 'fut>(state: &'s crate::component::state::State<Self>) -> futures::prelude::future::BoxFuture<'fut, Result<super::VNode, crate::error::Error>> where 's: 'fut {
            Box::pin(async {
                VNode::element(&state.scope, "div")
                .append_child(VNode::text(state.data.attr.clone()))
                .into()
            })      
        }
    }

    #[tokio::test]
    pub async fn test_foo_root() {
        let state = State::<Foo>::new_root(RenderMode::SSR, FooProps{}).await;
        let vnode = state.render().await.unwrap();
        println!("{}", vnode.render_to_string().await.unwrap())

    }
}

