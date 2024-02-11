use std::sync::RwLock;
use std::ops::{Deref, DerefMut};
use crate::{interaction::BoundInteraction, interface::Slot};

/// Track dependencies
pub struct Tracker<Matter>{   
    slot: Slot<Matter>,
    interactions: RwLock<Vec<BoundInteraction<Matter>>>
}

impl<Matter> Tracker<Matter> {
    /// Create a new tracker
    pub fn new(slot: Slot<Matter>) -> Self {
        Self{
            slot,
            interactions: RwLock::new(Vec::default())
        }
    }
 
    /// Track the current interaction and add it as a dep.
    pub fn track(&self) {
        if let Some(bint) = self.slot.current_interaction() {
            let mut ints = self.interactions.write().unwrap();
            ints.push(bint);
            ints.deref_mut().dedup();
        }
    }
   
    /// Trigger all interactions.
    pub fn trigger(&self) {
        self.interactions.read().unwrap().deref().iter().for_each(BoundInteraction::schedule);
    }
}
