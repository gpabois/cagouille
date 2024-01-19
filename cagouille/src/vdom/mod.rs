
use crate::component::traits::Component;

use self::{traits::RenderToStream, el::ElementNode, comp::ComponentNode, mode::Mode};
use futures::{io::AsyncWriteExt, future::LocalBoxFuture, AsyncWrite};
use seeded_random::{Seed, Random};

pub mod mode;

mod attr;
mod el;
mod comp;
mod node_ref;
mod node_key;


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
pub struct Scope {
    /// Node key
    pub id: node_key::VNodeKey,

    /// The rng generator for children node keys.
    pub(self) rng: Random,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new_root()
    }
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Self::new(self.id.0)
    }
}

impl Scope {
    /// New root scope
    pub fn new_root() -> Self {
        Self { id: Default::default(), rng: Random::from_seed(Seed::unsafe_new(0)) }
    }
    
    /// Create a new rendering scope
    pub fn new(id: u32) -> Self {
        Self { 
            id: id.into(), 
            rng: Random::from_seed(Seed::unsafe_new(id.into()))  
        }
    }

    /// Create a child scope
    pub fn new_child_scope(&self) -> Scope {
        Self::new(self.rng.u32())
    }
}

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

pub enum VNode<M> where M: Mode {
    Component(comp::AnyComponentNode<M>),
    Element(el::ElementNode<M>),
    Text(String),
    Empty
}

impl<M> VNode<M> where M: Mode {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn text<IntoStr: Into<String>>(text: IntoStr) -> Self {
        Self::Text(text.into())
    }

    pub fn element<IntoStr: Into<String>>(parent: &Scope, tag: IntoStr) -> ElementNode<M> {
        ElementNode::new(parent, tag)
    }

    pub fn component<Comp: Component<M>>(parent: &Scope, props: Comp::Properties, events: Comp::Events) -> ComponentNode<M, Comp> {
        ComponentNode::new(parent, props, events)
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
    use futures::future::LocalBoxFuture;

    use crate::{
        component::{traits::Component, ctx::Context, event::{EventSlot, traits::{EventSignal, Event}}}, 
        vdom::{Scope, mode::DebugMode, traits::RenderToStream}
    };

    use super::{VNode, mode::Mode};

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
    }

    pub struct FooData{}

    #[derive(Default)]
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
    }

    #[tokio::test]
    pub async fn test_foo_root() {
        let root_scope = Scope::new_root();

        let root: VNode<_> = VNode::<DebugMode>::component::<Foo>(
            &root_scope, 
            FooProps{}, 
            ()
        ).into();
        println!("{}", root.render_to_string().await.unwrap())

    }
}

