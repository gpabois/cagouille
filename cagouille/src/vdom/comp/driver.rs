use std::{any::Any, fmt::Debug};

use futures::future::LocalBoxFuture;

use crate::error::Error;

/// Bridge from the erased-type component node, and the implementation.
pub(super) trait CompDriver: Any + Debug {
    /// Initialise the component
    fn initialise<'fut>(&'fut mut self) -> LocalBoxFuture<'fut, Result<(), Error>>;
}
