use std::ops::Deref;
use tokio::sync::watch;

use crate::local::BoundInteraction;

pub struct SlotTx {
    current_interaction_tx: watch::Sender<Option<BoundInteraction>>
}

impl SlotTx {
    pub fn set_current_interaction(&mut self, bound: BoundInteraction) {
        self.current_interaction_tx.send(Some(bound)).unwrap()
    }

    pub fn pop_current_interaction(&mut self) {
        self.current_interaction_tx.send(None).unwrap()
    }
}

#[derive(Clone)]
/// Receive info from the reactor
pub struct Slot {
    current_interaction_rx: watch::Receiver<Option<BoundInteraction>>,
}

impl Slot {
    pub fn new(current_interaction_rx: watch::Receiver<Option<BoundInteraction>>) -> Self {
        Self { current_interaction_rx }
    }

    pub fn create() -> (Slot, SlotTx) {
        let (current_interaction_tx, current_interaction_rx) =  watch::channel(None);

        (
            Slot {current_interaction_rx},
            SlotTx {current_interaction_tx}
        )
    }

    /// Returns the current bound interactions, if any.
    pub fn current_interaction(&self) -> Option<BoundInteraction> {
        self.current_interaction_rx.borrow().deref().clone()
    }
}
