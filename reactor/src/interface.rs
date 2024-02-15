use crate::{interaction::BoundInteraction, reaction::AnyReaction};
use std::ops::Deref;
use tokio::sync::{mpsc, watch};

#[derive(Clone)]
/// Type-erased signal
pub(crate) struct Signal {
    reactions: mpsc::UnboundedSender<AnyReaction>,
}

impl Signal {
    pub fn new(reactions: mpsc::UnboundedSender<AnyReaction>) -> Self {
        Self { reactions }
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        self.reactions.same_channel(&other.reactions)
    }
}

impl Signal {
    /// Send a reaction to the reactor
    pub fn send<I: Into<AnyReaction>>(&self, into_reaction: I) {
        self.reactions.send(into_reaction.into()).unwrap();
    }
}

#[derive(Clone)]
/// Receive info from the reactor
pub(crate) struct Slot {
    /// Current bound interaction
    current: watch::Receiver<Option<BoundInteraction>>,
}

impl Slot {
    pub fn new(current: watch::Receiver<Option<BoundInteraction>>) -> Self {
        Self { current }
    }

    /// Returns the current bound interactions, if any.
    pub fn current_interaction(&self) -> Option<BoundInteraction> {
        self.current.borrow().deref().clone()
    }
}
