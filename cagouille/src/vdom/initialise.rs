use futures::future::LocalBoxFuture;

use super::{VNode, VNodeData};


impl VNode {
    /// Initialise the virtual tree
    pub fn initialise<'fut>(&'fut mut self) -> LocalBoxFuture<'fut, ()> {
        Box::pin(self.data.initialise())
    }
}

impl VNodeData {
    pub async fn initialise(&mut self) {
        match self {
            Self::Component(comp) => comp.initialise().await,
            Self::Element(el) => el.initialise().await,
            _ => {}
        }
    }
}