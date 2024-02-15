use crate::{
    interface::Signal,
    reaction::{AnyReaction, Reaction},
    Context,
};
use std::{any::Any, sync::Arc};

#[derive(Clone)]
/// Interaction bound to a reactor
pub(crate) struct BoundInteraction {
    /// The interaction
    interaction: AnyInteraction,
    /// Signal to send the interaction
    signal: Signal,
}

impl PartialEq for BoundInteraction {
    fn eq(&self, other: &Self) -> bool {
        self.interaction == other.interaction && self.signal == other.signal
    }
}

impl BoundInteraction {
    ///
    pub fn new(interaction: AnyInteraction, signal: Signal) -> Self {
        Self {
            interaction,
            signal,
        }
    }
    /// Send interaction to the reactor
    pub fn schedule(&self) {
        self.signal.send(self.interaction.clone());
    }
}

#[derive(Clone)]
/// Any interaction
pub struct AnyInteraction(Arc<dyn Any + Sync + Send>);

impl<Matter> From<Interaction<Matter>> for AnyInteraction
where
    Matter: 'static,
{
    fn from(value: Interaction<Matter>) -> Self {
        Self(value.0)
    }
}

impl Into<AnyReaction> for AnyInteraction {
    fn into(self) -> AnyReaction {
        AnyReaction::Interact(self)
    }
}

impl PartialEq for AnyInteraction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(Arc::as_ptr(&self.0), Arc::as_ptr(&other.0))
    }
}

impl AnyInteraction {
    pub fn downcast<Matter>(self) -> Option<Interaction<Matter>>
    where
        Matter: 'static,
    {
        match self.0.downcast::<InteractionInner<Matter>>() {
            Ok(inner) => Some(Interaction(inner)),
            Err(_) => None,
        }
    }
}

pub type InteractionFn<Matter> = dyn Fn(Context<'_, Matter>) + Sync + Send + 'static;

struct InteractionInner<Matter>(Box<InteractionFn<Matter>>);

/// An interaction is a function run by the reactor's core
pub struct Interaction<Matter>(Arc<InteractionInner<Matter>>);

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

impl<Matter> Into<AnyReaction> for Interaction<Matter>
where
    Matter: 'static,
{
    fn into(self) -> AnyReaction {
        let any_interaction: AnyInteraction = self.into();
        any_interaction.into()
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
        Self(Arc::new(InteractionInner(Box::new(f))))
    }

    /// Execute the interaction
    pub fn execute(&self, context: Context<'_, Matter>) {
        self.0 .0(context);
    }
}
