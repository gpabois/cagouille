use crate::{
    action::{Action, AnyAction},
    interaction::AnyInteraction,
    Context, Interaction,
};

///  A reactor's command
pub enum Reaction<Matter> {
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

pub enum AnyReaction {
    Interact(AnyInteraction),
    Act(AnyAction),
}

impl AnyReaction {
    pub fn downcast<Matter>(self) -> Option<Reaction<Matter>>
    where
        Matter: Send + Sync + 'static,
    {
        match self {
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
            Reaction::Interact(interaction) => Self::Interact(interaction.into()),
            Reaction::Act(action) => Self::Act(action.into()),
        }
    }
}
