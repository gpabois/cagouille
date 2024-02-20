use crate::sync::{Action, AnyAction, AnyInteraction, BoundInteraction, Interaction};
use super::Reaction;

pub enum AnyReaction {
    BoundInteract(BoundInteraction),
    Interact(AnyInteraction),
    Act(AnyAction),
}

impl AnyReaction {
    pub fn downcast<Matter>(self) -> Option<Reaction<Matter>>
    where
        Matter: Sync + Send + 'static,
    {
        match self {
            AnyReaction::BoundInteract(any) => Some(Reaction::BoundInteract(any)),
            AnyReaction::Interact(any) => any
                .downcast::<Matter>()
                .map(|interaction| interaction.into()),
            AnyReaction::Act(any) => any.downcast::<Matter>().map(|action| action.into()),
        }
    }
}

impl<Matter> From<Reaction<Matter>> for AnyReaction
where
    Matter: Sync + Send + 'static,
{
    fn from(value: Reaction<Matter>) -> Self {
        match value {
            Reaction::BoundInteract(bound_interaction) => Self::BoundInteract(bound_interaction),
            Reaction::Interact(interaction) => Self::Interact(interaction.into()),
            Reaction::Act(action) => action.into(),
        }
    }
}

impl From<BoundInteraction> for AnyReaction {
    fn from(value: BoundInteraction) -> Self {
        Self::BoundInteract(value)
    }
}

impl<Matter> From<Interaction<Matter>> for AnyReaction
where
    Matter: Sync + Send +'static,
{
    fn from(value: Interaction<Matter>) -> Self {
        Self::Interact(value.into())
    }
}

impl<Matter> From<Action<Matter>> for AnyReaction
where
    Matter: Sync + Send +'static,
{
    fn from(value: Action<Matter>) -> Self {
        Self::Act(value.into())
    }
}
