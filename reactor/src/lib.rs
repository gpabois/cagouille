mod action;
mod r#async;
mod atom;
mod conf;
mod context;
mod core;
mod interaction;
mod interface;
mod measure;
mod pilot;
mod ray;
mod reaction;
mod scheduler;
mod tracker;

pub use atom::Atom;
pub use context::Context;
pub use context::InitContext;
pub use interaction::Interaction;
pub use measure::Measure;
use r#async::BoxFuture;
use r#async::MaybeAsync;
pub use ray::Ray;

use pilot::Pilot;
use reaction::Reaction;

pub struct Reactor<Matter>(Pilot<Matter>)
where
    Matter: 'static;

impl<Matter> Reactor<Matter>
where
    Matter: Sync + Send + 'static,
{
    pub fn new<F>(init: F) -> Self
    where
        F: FnOnce(InitContext<Matter>) -> Matter,
    {
        let f = MaybeAsync::Sync(init);
        Self(core::Core::create(f))
    }

    pub fn async_new<F>(init: F) -> Self
    where
        F: FnOnce(InitContext<Matter>) -> BoxFuture<'static, Matter> + 'static,
    {
        let f = MaybeAsync::Async(Box::new(init));
        Self(core::Core::create(f))
    }

    /// Use a measure
    pub fn use_measure<D, F>(&self, init: D, f: F) -> Measure<D>
    where
        D: Sync + Send,
        F: Fn(Context<Matter>) -> D + Sync + Send + 'static,
    {
        Measure::new(init, f, self.0.get_signal())
    }

    /// Use a measure, wait for it to stabilise (computed at least once)
    pub async fn use_stabilised_measure<D, F>(&self, init: D, f: F) -> Measure<D>
    where
        D: Sync + Send,
        F: Fn(Context<Matter>) -> D + Sync + Send + 'static,
    {
        let mut measure = self.use_measure(init, f);
        measure.changed().await;
        measure
    }

    /// Perform an action
    pub fn act<F>(&self, f: F)
    where
        F: FnOnce(Context<Matter>) + Sync + Send + 'static,
    {
        self.0.get_signal().send(Reaction::act(f));
    }

    pub async fn shutdown(self) {
        self.0.shutdown().await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Atom, Ray, Reactor};
    use std::time::Duration;

    #[tokio::test]
    /// Test measure, without any reaction.
    pub async fn test_measure_no_reaction() {
        pub struct Foo {
            atom: Atom<bool>,
        }

        let reactor = Reactor::<Foo>::new(|ctx| Foo {
            atom: ctx.use_atom(true),
        });
        // Create a simple measure of data.
        let measure = reactor.use_stabilised_measure(false, |ctx| *ctx.atom).await;
        assert!(measure.to_owned());
    }

    #[tokio::test]
    pub async fn test_no_multiple_interaction_trigger() {
        pub struct Foo {
            a0: Atom<bool>,
            a1: Atom<bool>,
            a2: Atom<u8>,
            r0: Ray<bool>,
        }

        let reactor = Reactor::<Foo>::new(|ctx| Foo {
            a0: ctx.use_atom(false),
            a1: ctx.use_atom(false),
            a2: ctx.use_atom(0),
            r0: ctx.use_ray(true, |mut ctx| {
                *ctx.a2 += 1;
                *ctx.a0 && *ctx.a1
            }),
        });

        // Create a simple measure of data.
        let mut m0 = reactor
            .use_stabilised_measure(false, |ctx| ctx.r0.to_owned())
            .await;
        let mut m1 = reactor
            .use_stabilised_measure(0, |ctx| ctx.a2.to_owned())
            .await;

        // Modify two deps, it should call ray's function only once.
        reactor.act(|mut ctx| {
            *ctx.a0 = true;
            *ctx.a1 = true;
        });

        m0.changed().await;
        m1.changed().await;

        // Should not have changed again.
        m1.changed_or_timeout(Duration::from_millis(100)).await;

        assert!(m0.to_owned());
        assert!(m1.to_owned() == 2);
    }

    #[tokio::test]
    /// Test ray
    /// Scenario:
    /// The ray's value is the neg of the atom's value.
    /// The atom's value is modified through an action, it should reflect onto the ray's value.
    /// A measure is used to retrieve the ray's value.
    pub async fn test_ray() {
        pub struct Foo {
            atom: Atom<bool>,
            ray: Ray<bool>,
        }

        let reactor = Reactor::<Foo>::new(|ctx| Foo {
            atom: ctx.use_atom(true),
            ray: ctx.use_ray(true, |ctx| !*ctx.atom),
        });

        // Wait for the measure to stabilise
        let mut measure = reactor
            .use_stabilised_measure(false, |ctx| ctx.ray.to_owned())
            .await;

        assert!(!measure.to_owned());

        // Modify the atom, should trigger a ray update.
        reactor.act(|mut ctx| *ctx.atom = false);

        // Wait for measure update.
        measure.changed().await;

        //
        assert!(measure.to_owned());
    }
}
