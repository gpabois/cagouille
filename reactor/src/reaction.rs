use crate::{action::Action, Context, Interaction};

///  A reactor's command
pub enum Reaction<Matter> {
    Interact(Interaction<Matter>),
    Act(Action<Matter>)
}

impl<Matter> Reaction<Matter> {
    pub fn interact<F: Fn(Context<Matter>) + Sync + Send + 'static>(f: F) -> Self {
        Self::Interact(Interaction::new(f))
    }

    pub fn act<F>(f: F) -> Self 
    where F: FnOnce(Context<Matter>) + Sync + Send + 'static
    {
        Self::Act(Action::new(f))
    }
}
