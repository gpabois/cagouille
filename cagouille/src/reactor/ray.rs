use std::{cell::RefCell, rc::Rc};

use super::tracker::Tracker;

pub struct Ray<Matter, D> {
    value:   D,
    tracker: Tracker<Matter>
}

impl<Matter, D> Ray<Matter, D> {
    pub fn new(value: D, tracker: Tracker<Matter>) -> Self {
        Self {
            value,
            tracker
        }
    }

    /// Borrow the value
    pub fn borrow(&self) -> &D {
        self.tracker.track();
        return &self.value;
    }

    /// Borrow mut
    pub fn borrow_mut(&mut self) -> &mut D {
        self.tracker.trigger();
        return &mut self.value;
    }
}