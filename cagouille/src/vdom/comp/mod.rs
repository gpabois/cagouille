use crate::component::{traits::Component, State};
use futures::{future::LocalBoxFuture, AsyncWrite};
use std::any::TypeId;

use super::{mode::Mode, node_key::VNodeKey, traits::RenderToStream};

pub mod concrete;
pub mod df;
pub mod driver;

pub use concrete::ConcreteComponentNode;

type Driver<M> = dyn driver::CompDriver<M> + Send + Sync;

/// Node component
pub struct ComponentNode<M: Mode> {
    /// Drive the component node impl.
    driver: Box<Driver<M>>,
}

impl<M: Mode> ComponentNode<M> {
    pub fn new<Comp>(props: Comp::Properties, events: Comp::Events) -> Self
    where
        Comp: Component<M> + 'static,
    {
        Self {
            driver: Box::new(ConcreteComponentNode::<M, Comp>::new(props, events)),
        }
    }

    /// Initialise the component node.
    pub async fn initialise(&mut self) {
        self.driver
            .initialise()
            .await
            .expect("cannot initialise component");
    }

    /// Returns the node
    pub fn type_id(&self) -> TypeId {
        self.driver.type_id()
    }
}

impl<'a, M> RenderToStream<'a> for &'a ComponentNode<M>
where
    M: Mode,
{
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + futures::AsyncWriteExt + Unpin>(
        self,
        stream: &'stream mut W,
    ) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where
        'a: 'fut,
        'stream: 'fut,
    {
        Box::pin(async {
            self.v_node
                .0
                .read()
                .await
                .as_ref()
                .expect("vnode has not been rendered")
                .render_to_stream(stream)
                .await
        })
    }
}
