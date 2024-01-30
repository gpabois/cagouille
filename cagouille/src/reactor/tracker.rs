use std::{rc::Rc, cell::RefCell};
use super::{reactor::Current, Interaction};

type Interations<Matter> = Vec<Interaction<Matter>>;

/// Track the effects depending on the ref.
pub(crate) struct Tracker<Matter> {
    current: Current<Matter>,
    effects: Vec<Interaction<Matter>>
}

impl<Matter> Tracker<Matter> {
    pub fn new(current: Current<Matter>) -> Self {
        Self {
            current,
            effects: Default::default()
        }
    }

    fn register(&self, effect: Effect) {
        self.effects.borrow_mut().push(effect);
    }  

    /// Track the current effect and add it to the effects
    pub fn track(&self) {
        let maybe_current_effect = self.current.interaction();
        
        if maybe_current_effect.is_none() {
            return;
        }
        
        let current_effect = maybe_current_effect.unwrap();
        self.register(current_effect);
    }   
    
    /// Trigger all effects
    pub fn trigger(&self) {
        let effects = self.effects.clone();
        tokio::task::spawn_local(async move {
            effects.borrow().iter().for_each(Effect::call);
        });
    }

}
