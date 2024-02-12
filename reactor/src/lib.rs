mod action;
mod r#async;
mod atom;
mod context;
mod core;
mod interaction;
mod interface;
mod measure;
mod pilot;
mod ray;
mod reaction;
mod tracker;

pub use atom::Atom;
pub use context::Context;
pub use context::InitContext;
pub use interaction::Interaction;
use r#async::BoxFuture;
pub use ray::Ray;

use measure::Measure;
use pilot::Pilot;
use reaction::Reaction;

pub struct Reactor<Matter>(Pilot<Matter>);

impl<Matter> Reactor<Matter>
where
    Matter: Send + 'static,
{
    pub fn new<F>(init: F) -> Self
    where
        F: (FnOnce(InitContext<Matter>) -> Matter) + Sync + Send + 'static,
    {
        Self(core::Core::create(init))
    }

    pub fn async_new<F>(init: F) -> Self
    where
        F: FnOnce(InitContext<Matter>) -> BoxFuture<'static, Matter> + Sync + Send + 'static,
    {
        Self(core::Core::async_create(init))
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

    pub struct Foo {
        atom: Atom<bool, Self>,
        ray: Ray<bool, Self>,
    }

    #[tokio::test]
    /// Test measure, without any reaction.
    pub async fn test_measure_no_reaction() {
        let reactor = Reactor::<Foo>::new(|ctx| Foo {
            atom: ctx.use_atom(true),
            ray: ctx.use_ray(true, |ctx| *ctx.atom),
        });
        // Create a simple measure of data.
        let measure = reactor.use_stabilised_measure(false, |ctx| *ctx.atom).await;
        assert!(measure.to_owned());
    }

    #[tokio::test]
    /// Test ray
    /// Scenario:
    /// The ray's value is the neg of the atom's value.
    /// The atom's value is modified through an action, it should reflect onto the ray's value.
    /// A measure is used to retrieve the ray's value.
    pub async fn test_ray() {
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
