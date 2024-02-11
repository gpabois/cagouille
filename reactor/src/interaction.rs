use std::sync::Arc;
use crate::{interface::Signal, reaction::Reaction, Context};

/// Interaction bound to a reactor
pub struct BoundInteraction<Matter> {
    interaction: Interaction<Matter>,
    /// Signal to send the interaction
    signal: Signal<Matter>
}

impl<Matter> PartialEq for BoundInteraction<Matter> {
    fn eq(&self, other: &Self) -> bool {
        self.interaction == other.interaction && self.signal == other.signal
    }
}

impl<Matter> BoundInteraction<Matter> {
    /// i
    pub fn new(interaction: Interaction<Matter>, signal: Signal<Matter>) -> Self {
        Self{interaction, signal}
    }
    /// Send interaction to the reactor
    pub fn schedule(&self) {
        self.signal.send(self.interaction.clone());
    }
}

impl<Matter> Clone for BoundInteraction<Matter> {
    fn clone(&self) -> Self {
        Self { interaction: self.interaction.clone(), signal: self.signal.clone() }
    }
}
/// An interaction is a function run by the reactor's core
pub struct Interaction<Matter>(Arc<dyn Fn(Context<'_, Matter>) + Sync + Send + 'static>);

impl<Matter> PartialEq for Interaction<Matter> {
    fn eq(&self, other: &Self) -> bool {
       Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl<Matter> Into<Reaction<Matter>> for Interaction<Matter> {
    fn into(self) -> Reaction<Matter> {
       Reaction::Interact(self) 
    }
}

impl<Matter> Clone for Interaction<Matter> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Matter> Interaction<Matter> {
    /// Create a new interaction
    pub fn new<F: Fn(Context<'_, Matter>) + Sync + Send + 'static>(f: F) -> Self {
        Self(Arc::new(f))   
    }

    /// Execute the interaction
    pub fn execute(&self, context: Context<'_, Matter>) {
        self.0(context);
    }
}
