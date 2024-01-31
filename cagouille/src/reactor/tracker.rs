use std::cell::RefCell;

use super::{reactor::Reaction, Interaction, Reactor};


/// Track the effects depending on the ref.
pub(crate) struct Tracker<Matter> {
    reactor: Reactor<Matter>,
    interactions: RefCell<Vec<Interaction<Matter>>>
}

impl<Matter: Send + 'static> Tracker<Matter> {
    pub fn new(reactor: Reactor<Matter>) -> Self {
        Self {
            reactor,
            interactions: Default::default()
        }
    }

    fn register(&self, interaction: Interaction<Matter>) {
        self.interactions.borrow_mut().push(interaction);
    }  

    /// Track the current effect and add it to the effects
    pub fn track(&self) {
        let maybe_current_effect = self.reactor.current_interaction();
        
        if maybe_current_effect.is_none() {
            return;
        }
        
        let current_effect = maybe_current_effect.unwrap();
        self.register(current_effect);
    }   
    
    /// Trigger all effects
    pub fn trigger(&self) {
        self.interactions.borrow()
        .iter()
        .cloned()
        .for_each(|int| {
            self.reactor.react(Reaction::Interact(int)).unwrap();
        })
    }

}
