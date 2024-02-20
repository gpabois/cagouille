mod any;

use crate::local::{BoundInteraction, Interaction, Action, Context};

pub use any::AnyReaction;

///  A reactor's command
pub enum Reaction<Matter> {
    BoundInteract(BoundInteraction),
    Interact(Interaction<Matter>),
    Act(Action<Matter>),
}

impl<Matter> From<Interaction<Matter>> for Reaction<Matter> {
    fn from(value: Interaction<Matter>) -> Self {
        Self::Interact(value)
    }
}

impl<Matter> From<Action<Matter>> for Reaction<Matter> {
    fn from(value: Action<Matter>) -> Self {
        Self::Act(value)
    }
}

impl<Matter> Reaction<Matter>
where
    Matter: 'static,
{
    pub fn interact<F: Fn(Context<Matter>) + 'static>(f: F) -> Self {
        Self::Interact(Interaction::new(f))
    }

    pub fn act<F>(f: F) -> Self
    where
        F: FnOnce(Context<Matter>) + 'static,
    {
        Self::Act(Action::new(f))
    }
}
