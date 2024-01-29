use std::sync::Arc;

use futures::{future::LocalBoxFuture, Future};

#[derive(Clone)]
pub struct Effect<'eff>(Arc<dyn (Fn() -> LocalBoxFuture<'static, ()>) + 'eff>);

impl<'a> Effect<'a> {
    pub fn new<F: Fn() -> LocalBoxFuture<'static, ()> + 'a>(value: F) -> Self {
        Self(Arc::new(value))
    }

    pub fn call(self) -> impl Future<Output = ()> + 'static {
        self.0()
    }
}