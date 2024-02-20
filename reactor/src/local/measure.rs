use std::{
    cell::RefCell, ops::DerefMut, rc::Rc, 
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration
};

use crate::local::{Signal, Context, Interaction};
use tokio::time;

struct MeasureInner<D> {
    counter: std::sync::atomic::AtomicUsize,
    value: RefCell<D>,
}

impl<D> MeasureInner<D> {
    /// Update the value.
    pub fn update(&self, value: D) {
        *self.value.borrow_mut() = value;
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    /// Wait until the current version is not the same as the given initial version
    pub async fn changed(&self, version: usize) -> usize {
        loop {
            let curr = self.counter.load(Ordering::SeqCst);

            if curr != version {
                return curr;
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
        std::mem::take(self.value.borrow_mut().deref_mut())
    }
}

/// A measure from the reactor
/// This allows for external systems to receive updated
/// upon reactor's state change
pub struct Measure<D> where
D: 'static
{
    inner: Rc<MeasureInner<D>>,
    version: usize
}


impl<D> Clone for Measure<D>
where
    D: 'static,
{
    fn clone(&self) -> Self {
        Self {
            version: self.version,
            inner: self.inner.clone()
        }
    }
}

impl<D> Measure<D>
where
    D: 'static,
{
    /// Create a new measure.
    pub(crate) fn new<Matter, F>(init: D, f: F, signal: Signal) -> Self
    where
        F: Fn(Context<Matter>) -> D + 'static,
        Matter: 'static,
    {
        let inner = Rc::new(MeasureInner {
            counter: AtomicUsize::new(0),
            value: RefCell::new(init),
        });

        let in1 = inner.clone();

        signal.send(Interaction::new(move |ctx| {
            in1.update(f(ctx));
        }));

        Self{inner, version: 0}
    }

    pub async fn changed(&mut self) {
        self.version = self.inner.changed(self.version).await;
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
    D: Default + 'static,
{
    pub fn take(&self) -> D {
        self.inner.take()
    }
}

impl<D> Measure<D>
where
    D: ToOwned<Owned = D> + 'static,
{
    pub fn to_owned(&self) -> D {
        self.inner.value.borrow().to_owned()
    }
}
