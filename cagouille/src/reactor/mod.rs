
pub mod reactor;
pub mod effect;
pub mod reactive;

pub use effect::Effect;
use futures::future::LocalBoxFuture;
pub use reactor::Reactor;
pub use reactive::Reactive;

pub async fn use_effect<'comp, F: Fn() -> LocalBoxFuture<'static, ()> + 'comp>(f: F, reactor: &Reactor<'comp>) {
    let effect = Effect::new(f);
    reactor.with_effect(&effect).await;
    effect.call().await;
}

#[cfg(test)]
mod test {
    use super::Reactor;
    use super::Reactive;
    use super::use_effect;

    #[tokio::test]
    pub async fn simple_test() {
        let reactor = Reactor::new();
        let data = Reactive::new(&reactor, 5);

        use_effect(|| {
            Box::pin(async {
                println!("react !")
            })
        }, &reactor).await;
    }
}