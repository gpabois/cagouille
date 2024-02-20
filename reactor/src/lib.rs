/// Reactor without Sync + Send constraint.
pub mod local;

/// Reactor with Sync + Send constraint.
pub mod sync;


#[cfg(all(feature = "local"))]
pub use local::{Reactor, Action, Atom, Measure, Interaction}

#[cfg(all(feature = "local"))]
pub mod local_api {
    use crate::local::{Reactor, InitContext};
    
    pub fn new_reactor<Matter, F>(init: F) -> Reactor<Matter> where F: FnOnce(InitContext<Matter>) -> Matter + 'static {
        Reactor::new::<yase::Spawner, Matter>(init)
    }

    pub fn async_new_reactor<Matter, F, Fut>(init: F) -> Reactor<Matter> 
        where
            F: FnOnce(InitContext<Matter>) -> Fut + 'static,
            Fut: Future<Output=Matter> + 'static {
                Reactor::async_new<yase::Spawner, Matter>(init)
            }
}
#[cfg(all(feature = "local"))]
pub use local_api::*;

#[cfg(all(feature = "sync"))]
pub use sync::{Reactor, Action, Atom, Measure, Interaction}


#[cfg(test)]
mod tests {
    use crate::sync::{Atom, Ray, Reactor};
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
        let measure = reactor.use_stabilised_measure(|ctx| *ctx.atom).await;
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
