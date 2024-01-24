
use std::any::TypeId;
use futures::{future::LocalBoxFuture, AsyncWrite};
use crate::component::{traits::Component, State};

use super::{mode::Mode, node_key::VNodeKey, traits::RenderToStream, SharedVNode};

pub mod driver;
pub mod concrete;
pub mod df;

pub use concrete::ComponentNode;

pub struct AnyComponentNode<M: Mode> {
    /// The root node of the component, shared with the impl.
    v_node: SharedVNode<M>,

    /// Drive the component node impl.
    driver: Box<dyn driver::ComponentNodeDriver<M>>
}

impl<M: Mode> AnyComponentNode<M> {
    /// Initialise the component node.
    pub async fn initialise(&self) {
        self
        .driver
        .initialise()
        .await
        .expect("cannot initialise component");
    }

    /// Returns the node key
    pub fn id(&self) -> &VNodeKey {
        self.driver.id()
    }

    /// Returns the node
    pub fn type_id(&self) -> TypeId {
        self.driver.type_id()
    }

    /// Get a the concrete state of the component, returns None if the type does not match.
    pub(super) fn state<Comp>(&self) -> Option<State<M, Comp>> where M: Mode, Comp: Component<M> + 'static {
        self.driver.state().downcast()
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

