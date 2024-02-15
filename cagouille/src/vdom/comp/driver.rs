use std::any::Any;
use std::any::TypeId;

use futures::future::LocalBoxFuture;

use super::df::AnyComponentDf;
use super::ComponentNode;
use crate::error::Error;
use crate::vdom::Mode;
use crate::vdom::VNodeKey;

/// Bridge from the erased-type component node, and the implementation.
pub(super) trait CompDriver<M>: Any
where
    M: Mode,
{
    /// Initialise the component
    fn initialise<'a, 'fut>(&'a mut self) -> LocalBoxFuture<'fut, Result<(), Error>>
    where
        'a: 'fut;

    /// Compute differential between two components
    fn df<'a, 'fut>(&'a self, other: &'a ComponentNode<M>) -> LocalBoxFuture<'fut, AnyComponentDf>
    where
        'a: 'fut;
}

