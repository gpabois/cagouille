use std::{ops::Deref, rc::Rc};
use tokio::sync::watch;
use crate::local::{
    Signal,
    Reaction,
    Tracker,
    Context
};

/// A ray is a computed read-only value
pub struct Ray<D>
where
    D: 'static,
{
    value: watch::Receiver<D>,
    tracker: Rc<Tracker>,
}

impl<D> Ray<D>
where
    D: 'static,
{
    pub(crate) fn new<Matter, F>(init: D, f: F, signal: Signal, tracker: Tracker) -> Self
    where
        F: Fn(Context<Matter>) -> D + 'static,
        Matter: 'static,
    {
        let (tx, rx) = watch::channel(init);
        let tracker = Rc::new(tracker);

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
    D: Clone + 'static,
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
