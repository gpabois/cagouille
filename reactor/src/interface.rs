use std::ops::Deref;
use tokio::sync::{mpsc, watch};
use crate::{interaction::BoundInteraction, reaction::Reaction};

/// Signal to send info to the reactor
pub struct Signal<Matter> {
    reactions: mpsc::UnboundedSender<Reaction<Matter>>
}

impl<Matter> Signal<Matter> {
    pub fn new(reactions: mpsc::UnboundedSender<Reaction<Matter>>) -> Self {
        Self{reactions}
    }
}

impl<Matter> PartialEq for Signal<Matter> {
    fn eq(&self, other: &Self) -> bool {
        self.reactions.same_channel(&other.reactions)
    }
}

impl<Matter> Clone for Signal<Matter> {
    fn clone(&self) -> Self {
        Self { reactions: self.reactions.clone() }
    }
}

impl<Matter> Signal<Matter> {
    /// Send a reaction to the reactor
    pub fn send<I: Into<Reaction<Matter>>>(&self, into_reaction: I) {
        self.reactions.send(into_reaction.into()).unwrap();       
    }
}


/// Receive info from the reactor
pub struct Slot<Matter> {
    /// Current bound interaction
    current: watch::Receiver<Option<BoundInteraction<Matter>>>
    
}

impl<Matter> Clone for Slot<Matter> {
    fn clone(&self) -> Self {
        Self { current: self.current.clone() }
    }
}

impl<Matter> Slot<Matter> {
    pub fn new(current: watch::Receiver<Option<BoundInteraction<Matter>>>) -> Self {
        Self{current}
    }

    /// Returns the current bound interactions, if any.
    pub fn current_interaction(&self) -> Option<BoundInteraction<Matter>> {
      self.current.borrow().deref().clone()  
    }
}
