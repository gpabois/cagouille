mod any;

use crate::sync::{BoundInteraction, Interaction, Action, Context};

pub use any::AnyReaction;

///  A reactor's command
pub enum Reaction<Matter> {
    BoundInteract(BoundInteraction),
    Interact(Interaction<Matter>),
    Act(Action<Matter>),
}

impl<Matter> From<Interaction<Matter>> for Reaction<Matter> 
{
    fn from(value: Interaction<Matter>) -> Self {
        Self::Interact(value)
    }
}

impl<Matter> From<Action<Matter>> for Reaction<Matter> 
{
    fn from(value: Action<Matter>) -> Self {
        Self::Act(value)
    }
}


impl<Matter> Reaction<Matter>
where
    Matter: Sync + Send + 'static,
{
    pub fn interact<F: Fn(Context<Matter>) + Sync + Send + 'static>(f: F) -> Self {
        Self::Interact(Interaction::new(f))
    }

    pub fn act<F>(f: F) -> Self
    where
        F: FnOnce(Context<Matter>) + Sync + Send + 'static,
    {
        Self::Act(Action::new(f))
    }
}