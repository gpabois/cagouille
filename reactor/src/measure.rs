use std::{
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

use crate::{interface::Signal, Context, Interaction};
use tokio::time;

struct MeasureInner<D> {
    counter: std::sync::atomic::AtomicUsize,
    value: RwLock<D>,
}

impl<D> MeasureInner<D> {
    /// Update the value.
    pub fn update(&self, value: D) {
        *self.value.write().unwrap() = value;
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    pub async fn changed(&self) {
        let curr = self.counter.load(Ordering::SeqCst);
        loop {
            if self.counter.load(Ordering::SeqCst) != curr {
                break;
            }
            tokio::task::yield_now().await;
        }
    }
}

impl<D> MeasureInner<D>
where
    D: Default,
{
    // Take the inner value
    pub fn take(&self) -> D {
        std::mem::take(self.value.write().unwrap().deref_mut())
    }
}

/// A measure from the reactor
/// This allows for external systems to receive updated
/// upon reactor's state change
pub struct Measure<D>(Arc<MeasureInner<D>>)
where
    D: Sync + Send + 'static;

impl<D> Clone for Measure<D>
where
    D: Sync + Send + 'static,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D> Measure<D>
where
    D: Sync + Send + 'static,
{
    /// Create a new measure.
    pub(crate) fn new<Matter, F>(init: D, f: F, signal: Signal) -> Self
    where
        F: Fn(Context<Matter>) -> D + Sync + Send + 'static,
        Matter: 'static,
    {
        let inner = Arc::new(MeasureInner {
            counter: AtomicUsize::new(0),
            value: RwLock::new(init),
        });

        let in1 = inner.clone();

        signal.send(Interaction::new(move |ctx| {
            in1.update(f(ctx));
        }));

        Self(inner)
    }

    pub async fn changed(&mut self) {
        self.0.changed().await;
    }

    pub async fn changed_or_timeout(&mut self, d: Duration) {
        let sleep = time::sleep(d);
        tokio::pin!(sleep);

        tokio::select! {
            _ = self.changed() => {},
            _ = &mut sleep => {}
        }
    }
}

impl<D> Measure<D>
where
    D: Sync + Send + Default + 'static,
{
    pub fn take(&self) -> D {
        self.0.take()
    }
}

impl<D> Measure<D>
where
    D: Sync + Send + ToOwned<Owned = D> + 'static,
{
    pub fn to_owned(&self) -> D {
        self.0.value.read().unwrap().deref().to_owned()
    }
}
