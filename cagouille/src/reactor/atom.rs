use std::ops::{Deref, DerefMut};

use super::tracker::Tracker;

pub struct Atom<Matter, D> {
    value:   D,
    tracker: Tracker<Matter>
}

impl<Matter, D> Deref for Atom<Matter, D> 
where Matter: Send + 'static
{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.tracker.track();
        return &self.value;
    }
}

impl<Matter, D> DerefMut for Atom<Matter, D> 
where Matter: Send + 'static
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tracker.trigger();
        return &mut self.value;
    }
}

impl<Matter: Send + 'static, D> Atom<Matter, D> {
    pub(super) fn new(value: D, tracker: Tracker<Matter>) -> Self {
        Self {
            value,
            tracker
        }
    }
}