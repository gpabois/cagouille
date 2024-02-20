use std::{ops::Deref, sync::Arc};

use tokio::sync::watch;

use crate::{interface::Signal, reaction::Reaction, tracker::Tracker, Context};

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
        let (sender, receiver) = watch::channel(init);
        let trck = Arc::new(tracker);

        let react_trck = trck.clone();

        signal.send(Reaction::interact(move |ctx| {
            sender.send(f(ctx)).unwrap();
            react_trck.trigger();
        }));

        Ray {
            value: receiver,
            tracker: trck,
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
    D: Send + Sync + Clone + 'static,
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
