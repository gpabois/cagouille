
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
    DOM,
    SSR,
    Hydration
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::DOM
    }
}

/// Node's scope
pub struct VNodeScope {
    /// Node key
    pub id: node_key::VNodeKey,
    
    /// Render mode
    pub mode: RenderMode, 

    /// The rng generator for children node keys.
    pub(self) rng: Random,
}

impl Default for VNodeScope {
    fn default() -> Self {
        Self::new_root(Default::default())
    }
}

impl Clone for VNodeScope {
    fn clone(&self) -> Self {
        Self::new(self.id.0, self.mode.clone())
    }
}

impl VNodeScope {
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
    pub fn new_child_scope(&self) -> VNodeScope {
        Self::new(self.rng.u32(), self.mode.clone())
    }
}

pub mod traits 
{
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

pub enum VNode {
    Component(comp::AnyComponentNode),
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

    pub fn element<IntoStr: Into<String>>(parent: &VNodeScope, tag: IntoStr) -> ElementNode {
        ElementNode::new(parent, tag)
    }

    pub fn component<Comp: Component>(parent: &VNodeScope, props: Comp::Properties) -> ComponentNode<Comp> {
        ComponentNode::new(parent, props)
    }
}

impl<'a> RenderToStream<'a> for &'a VNode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
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
    use futures::future::LocalBoxFuture;

    use crate::{component::{traits::Component, ctx::Context, event::{EventSlot, traits::{EventSignal, Event}}}, vdom::{VNodeScope, RenderMode, traits::{Renderable, RenderToStream}}};

    use super::VNode;

    pub struct BarData{attr: String}

    #[derive(Default)]
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

    impl<'a> Component for Bar<'a> {
        type Properties = BarProps;
        type Data = BarData;
        type Events = BarEvents<'a>;

        fn data<'props, 'fut>(props: &'props Self::Properties) -> LocalBoxFuture<'fut, Self::Data> where 'props: 'fut {
            Box::pin(std::future::ready(Self::Data{
                attr: props.attr.clone()
            }))
        }

        fn render<'s, 'fut>(ctx: Context<'s, Self>) -> LocalBoxFuture<'fut, Result<VNode, crate::error::Error>> where 's: 'fut {
            Box::pin(async move {
                VNode::element(&ctx.scope, "div")
                .append_child(VNode::text(&ctx.data.attr))
                .consume()
                .into()
            })
        }
    }

    pub struct FooData{}

    #[derive(Default)]
    pub struct FooProps{}

    pub struct Foo;

    impl Component for Foo {
        type Properties = FooProps;
        type Data = FooData;
        type Events = ();

        fn data<'props, 'fut>(props: &'props Self::Properties) -> LocalBoxFuture<'fut, Self::Data> where 'props: 'fut {
            Box::pin(std::future::ready(Self::Data{}))
        }

        fn render<'s, 'fut>(ctx: Context<'s, Self>) -> LocalBoxFuture<'fut, Result<super::VNode, crate::error::Error>> where 's: 'fut {
            Box::pin(async move {
                VNode::element(&ctx.scope, "div")
                .extend_child((0..10_000).map(|_| 
                    VNode::component::<Bar>(
                        &ctx.scope, 
                        BarProps{attr: "Hello world".into()}
                    )
                    .on(BarChanged, EventSlot::new(|payload| {
                        println!("{payload}")
                    }))
                    .consume()
                    .into()
                ))
                .consume()
                .into()
            })      
        }
    }

    #[tokio::test]
    pub async fn test_foo_root() {
        let root = VNode::component::<Foo>(&VNodeScope::new_root(RenderMode::SSR), FooProps{});
        let vnode = root.render().await.unwrap();
        println!("{}", vnode.render_to_string().await.unwrap())

    }
}

