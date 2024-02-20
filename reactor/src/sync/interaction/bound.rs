use crate::sync::Signal;
use super::{AnyInteraction, Interaction};

use std::sync::{Arc, RwLock};

struct Inner {
    interaction: AnyInteraction,
    signal: Signal,
    scheduled: RwLock<bool>,
}

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        self.interaction == other.interaction && self.signal == other.signal
    }
}

#[derive(Clone, PartialEq)]
/// Interaction bound to a reactor
pub struct BoundInteraction(Arc<Inner>);

impl BoundInteraction {
    ///
    pub fn new(interaction: AnyInteraction, signal: Signal) -> Self {
        Self(Arc::new(Inner {
            interaction,
            signal,
            scheduled: RwLock::new(false),
        }))
    }

    /// Send interaction to the reactor
    pub fn schedule(&self) {
        if !*self.0.scheduled.read().unwrap() {
            *self.0.scheduled.write().unwrap() = true;
            self.0.signal.send(self.clone());
        }
    }

    pub fn ack(&self) {
        *self.0.scheduled.write().unwrap() = false;
    }

    pub fn downcast<Matter>(&self) -> Option<Interaction<Matter>>
    where
        Matter: Sync + Send + 'static,
    {
        self.0.interaction.clone().downcast()
    }
}

