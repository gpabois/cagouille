use std::{ops::Deref, sync::Arc};
use tokio::sync::watch;
use crate::sync::{
    Signal,
    Reaction,
    Tracker,
    Context
};

/// A ray is a computed read-only value
pub struct Ray<D>
where
    D: Sync + Send + 'static,
{
    value: watch::Receiver<D>,
    tracker: Arc<Tracker>,
}

impl<D> Ray<D>
where
    D: Sync + Send + 'static,
{
    pub(crate) fn new<Matter, F>(init: D, f: F, signal: Signal, tracker: Tracker) -> Self
    where
        F: Fn(Context<Matter>) -> D + Sync + Send + 'static,
        Matter: Sync + Send + 'static,
    {
        let (tx, rx) = watch::channel(init);
        let tracker = Arc::new(tracker);

        let interaction_tracker = tracker.clone();

        signal.send(Reaction::interact(move |ctx| {
            tx.send(f(ctx)).unwrap();
            interaction_tracker.trigger();
        }));

        Ray {
            value: rx,
            tracker,
        }
    }

    /// Borrow the current ray's value.
    pub fn borrow(&self) -> Ref<'_, D> {
        self.tracker.track();
        Ref(self.value.borrow())
    }
}

impl<D> Ray<D>
where
    D: Clone + Sync + Send + 'static,
{
    pub fn to_owned(&self) -> D {
        self.borrow().clone()
    }
}

pub struct Ref<'a, D>(watch::Ref<'a, D>);

impl<'a, D> Deref for Ref<'a, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
