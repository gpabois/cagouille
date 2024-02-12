use std::{future::Future, pin::Pin};

pub type BoxFuture<'a, R> = Pin<Box<dyn Future<Output = R> + Sync + Send + 'a>>;

pub enum MaybeAsync<Args, Ret> {
    Sync(Box<dyn FnOnce(Args) -> Ret + Sync + Send + 'static>),
    Async(Box<dyn FnOnce(Args) -> BoxFuture<'static, Ret> + Sync + Send + 'static>),
}

impl<Args, Ret> MaybeAsync<Args, Ret> {
    /// Call the routine
    pub async fn call(self, args: Args) -> Ret {
        match self {
            MaybeAsync::Sync(f) => f(args),
            MaybeAsync::Async(f) => f(args).await,
        }
    }
}
