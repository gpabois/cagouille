use crate::local::Signal;
use super::{AnyInteraction, Interaction};

use std::{
    rc::Rc,
    cell::RefCell
};

struct Inner {
    interaction: AnyInteraction,
    signal: Signal,
    /// The bound interaction is scheduled to be executed
    scheduled: RefCell<bool>,
}

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        self.interaction == other.interaction && self.signal == other.signal
    }
}

#[derive(Clone, PartialEq)]
/// Interaction bound to a reactor
pub struct BoundInteraction(Rc<Inner>);

impl BoundInteraction {
    ///
    pub fn new(interaction: AnyInteraction, signal: Signal) -> Self {
        Self(Rc::new(Inner {
            interaction,
            signal,
            scheduled: RefCell::new(false),
        }))
    }

    /// Send interaction to the reactor
    pub fn schedule(&self) {
        if !*self.0.scheduled.borrow() {
            *self.0.scheduled.borrow_mut() = true;
            self.0.signal.send(self.clone());
        }
    }

    pub fn ack(&self) {
        *self.0.scheduled.borrow_mut() = false;
    }

    pub fn downcast<Matter>(&self) -> Option<Interaction<Matter>>
    where
        Matter: 'static,
    {
        self.0.interaction.clone().downcast()
    }
}

