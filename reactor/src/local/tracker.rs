use crate::local::{Slot, BoundInteraction};
use std::cell::RefCell;

/// Track dependencies
pub(crate) struct Tracker {
    slot: Slot,
    interactions: RefCell<Vec<BoundInteraction>>,
}

impl Tracker {
    /// Create a new tracker
    pub(crate) fn new(slot: Slot) -> Self {
        Self {
            slot,
            interactions: RefCell::new(Vec::default()),
        }
    }

    /// Track the current interaction and add it as a dep.
    pub fn track(&self) {
        if let Some(bint) = self.slot.current_interaction() {
            let mut ints = self.interactions.borrow_mut();
            ints.push(bint);
            ints.dedup();
        }
    }

    /// Trigger all interactions.
    pub fn trigger(&self) {
        self.interactions
            .borrow()
            .iter()
            .for_each(BoundInteraction::schedule);
    }
}
