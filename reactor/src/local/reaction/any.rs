use crate::local::{BoundInteraction, Interaction, AnyInteraction, AnyAction, Action};
use super::Reaction;

pub enum AnyReaction {
    BoundInteract(BoundInteraction),
    Interact(AnyInteraction),
    Act(AnyAction),
}

impl AnyReaction {
    pub fn downcast<Matter>(self) -> Option<Reaction<Matter>>
    where
        Matter: 'static,
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
    Matter: 'static,
{
    fn from(value: Reaction<Matter>) -> Self {
        match value {
            Reaction::BoundInteract(bound_interaction) => Self::BoundInteract(bound_interaction),
            Reaction::Interact(interaction) => Self::Interact(interaction.into()),
            Reaction::Act(action) => Self::Act(action.into()),
        }
    }
}

impl From<BoundInteraction> for AnyReaction {
    fn from(value: BoundInteraction) -> Self {
        Self::BoundInteract(value)
    }
}

impl From<AnyInteraction> for AnyReaction {
    fn from(value: AnyInteraction) -> Self {
        Self::Interact(value)
    }
}

impl<Matter> From<Interaction<Matter>> for AnyReaction
where
    Matter: 'static,
{
    fn from(value: Interaction<Matter>) -> Self {
        Self::Interact(value.into())
    }
}

impl<Matter> From<Action<Matter>> for AnyReaction
where
    Matter: 'static,
{
    fn from(value: Action<Matter>) -> Self {
        Self::Act(value.into())
    }
}

