use crate::{
    action::{Action, AnyAction},
    interaction::{AnyInteraction, BoundInteraction},
    Context, Interaction,
};

///  A reactor's command
pub(crate) enum Reaction<Matter> {
    BoundInteract(BoundInteraction),
    Interact(Interaction<Matter>),
    Act(Action<Matter>),
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

impl<Matter> From<Action<Matter>> for Reaction<Matter> {
    fn from(value: Action<Matter>) -> Self {
        Self::Act(value)
    }
}

impl<Matter> From<Interaction<Matter>> for Reaction<Matter> {
    fn from(value: Interaction<Matter>) -> Self {
        Self::Interact(value)
    }
}

pub(crate) enum AnyReaction {
    BoundInteract(BoundInteraction),
    Interact(AnyInteraction),
    Act(AnyAction),
}

impl AnyReaction {
    pub fn downcast<Matter>(self) -> Option<Reaction<Matter>>
    where
        Matter: Send + Sync + 'static,
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

pub mod local {}
