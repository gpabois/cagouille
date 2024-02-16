use std::any::Any;
use std::any::TypeId;

use futures::future::LocalBoxFuture;

use super::ComponentNode;
use crate::error::Error;

/// Bridge from the erased-type component node, and the implementation.
pub(super) trait CompDriver: Any
where {
    /// Initialise the component
    fn initialise<'a, 'fut>(&'a mut self) -> LocalBoxFuture<'fut, Result<(), Error>>
    where
        'a: 'fut;

}

