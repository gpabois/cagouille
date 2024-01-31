use std::sync::Arc;

use async_std::sync::{RwLock, RwLockReadGuard};
use futures::future::LocalBoxFuture;
use tokio::{join, task::JoinSet};

pub use super::{Effect, Reactor};

#[derive(Default, Clone)]
struct EffectTracker<'a>(Arc<RwLock<Vec<Effect<'a>>>>);

struct EffectTrackerReadGuard<'guard, 'eff>(RwLockReadGuard<'guard, Vec<Effect<'eff>>>);

impl<'a, 'eff> EffectTrackerReadGuard<'a, 'eff> {
    pub fn iter(&self) -> impl Iterator<Item=&Effect<'eff>> {
        self.0.iter()
    }
}

impl<'comp> EffectTracker<'comp> {
    pub async fn push(&self, effect: Effect<'comp>) {
        self.0.write().await.push(effect);
    }

    pub async fn read(&self) -> EffectTrackerReadGuard<'_, 'comp> {
        EffectTrackerReadGuard(self.0.read().await)
    }
}

pub struct Reactive<'comp, D> {
    inner: D,
    reactor: Reactor<'comp>,
    tracker: EffectTracker<'comp>
}

impl<'comp, D> Reactive<'comp, D> {
    pub fn new(reactor: &Reactor<'comp>, value: D) -> Self {
        Self {
            inner: value,
            reactor: reactor.clone(),
            tracker: Default::default()
        }
    }

    async fn deref(&self) -> &D {
        self.track().await;
        return &self.inner
    }

    async fn deref_mut(&mut self) -> &mut D {
        
    }

    /// Track the current effect and add it to the tracker list.
    fn track(&self) {
        let weak = self.tracker.downgrade();
        let reactor = self.reactor.clone();

        tokio::spawn(async move {
            let maybe_tracker = tracker.upgrade();
            if maybe_tracker.is_none() {
                return;
            }
            let mut tracker = maybe_tracker.unwrap();

            if let Some(effect) = reactor.current_effect().await {
                self.tracker.push(effect).await;
            }
        })

    }

    /// Notify the effects
    async fn notify(&self) {
        let mut set = JoinSet::new();

        self
        .tracker
        .read()
        .await
        .iter()
        .cloned()
        .for_each(|b| {
            let fut = b.call();
            set.spawn_local(fut);
        });

        while let Some(_) = set.join_next().await {}
    }
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