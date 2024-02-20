use std::ops::{Deref, DerefMut};
use crate::sync::Tracker;

/// Atom is a value within a reactor
/// If the value changes, it will notify all the interactions depending on it
pub struct Atom<D> {
    value: D,
    tracker: Tracker,
}

impl<D> Atom<D> {
    /// Creates a new atom
    pub(crate) fn new(value: D, tracker: Tracker) -> Self {
        Self { value, tracker }
    }
}

impl<D> Deref for Atom<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.tracker.track();
        &self.value
    }
}

impl<D> DerefMut for Atom<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tracker.trigger();
        &mut self.value
    }
}
