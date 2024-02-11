use std::ops::{Deref, DerefMut};

use crate::tracker::Tracker;

/// Atom is a value within a reactor
/// If the value changes, it will notify all the interactions depending on it
pub struct Atom<D, Matter> {
    value: D,
    tracker: Tracker<Matter>
}

impl<D, Matter> Atom<D, Matter> {
    /// Creates a new atom
    pub fn new(value: D, tracker: Tracker<Matter>) -> Self {
        Self{
            value, 
            tracker
        }
    }
}

impl<D, Matter> Deref for Atom<D, Matter> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.tracker.track();
        &self.value
    }
}

impl<D, Matter> DerefMut for Atom<D, Matter> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tracker.trigger();
        &mut self.value 
    }
}
