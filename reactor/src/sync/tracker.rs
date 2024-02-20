use crate::sync::{Slot, BoundInteraction};
use std::sync::RwLock;

/// Track dependencies
pub(crate) struct Tracker {
    slot: Slot,
    interactions: RwLock<Vec<BoundInteraction>>,
}

impl Tracker {
    /// Create a new tracker
    pub(crate) fn new(slot: Slot) -> Self {
        Self {
            slot,
            interactions: RwLock::new(Vec::default()),
        }
    }

    /// Track the current interaction and add it as a dep.
    pub fn track(&self) {
        if let Some(bint) = self.slot.current_interaction() {
            let mut ints = self.interactions.write().unwrap();
            ints.push(bint);
            ints.dedup();
        }
    }

    /// Trigger all interactions.
    pub fn trigger(&self) {
        self.interactions
            .read()
            .unwrap()
            .iter()
            .for_each(BoundInteraction::schedule);
    }
}
