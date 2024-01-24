use std::any::TypeId;

use futures::future::LocalBoxFuture;

use crate::component::state::AnyState;
use crate::vdom::Mode;
use crate::vdom::VNodeKey;

use super::AnyComponentNode;
use super::df::AnyComponentDf;

/// Bridge from the erased-type component node, and the implementation.
pub(super) trait ComponentNodeDriver<M> where M: Mode {
    /// Get key of the node.
    fn id(&self) -> &VNodeKey;

    /// Underlying component's type id.
    fn type_id(&self) -> TypeId;

    /// Initialise the component
    fn initialise<'a, 'fut>(&'a self) -> LocalBoxFuture<'fut, Result<(), crate::error::Error>> where 'a: 'fut;

    /// Compute differential between two components
    fn df<'a, 'fut>(&'a self, other: &'a AnyComponentNode<M>) -> LocalBoxFuture<'fut, AnyComponentDf> where 'a: 'fut;

    /// Get an erased-type sharable reference to the state.
    fn state(&self) -> AnyState;
}