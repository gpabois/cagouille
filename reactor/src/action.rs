use std::any::Any;

use crate::{reaction::Reaction, Context};

pub struct AnyAction(Box<dyn Any + Send + Sync + 'static>);

impl AnyAction {
    pub fn downcast<Matter>(self) -> Option<Action<Matter>>
    where
        Matter: 'static,
    {
        match self.0.downcast::<Action<Matter>>() {
            Ok(boxed_action) => Some(*boxed_action),
            Err(_) => None,
        }
    }
}

impl<Matter> From<Action<Matter>> for AnyAction
where
    Matter: 'static,
{
    fn from(value: Action<Matter>) -> Self {
        Self(Box::new(value))
    }
}

pub type ActionFn<Matter> = dyn FnOnce(Context<Matter>) + Sync + Send + 'static;

/// An action in the reactor.
pub struct Action<Matter>(Box<ActionFn<Matter>>);

impl<Matter> Into<Reaction<Matter>> for Action<Matter> {
    fn into(self) -> Reaction<Matter> {
        Reaction::Act(self)
    }
}

impl<Matter> Action<Matter> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Context<Matter>) + Sync + Send + 'static,
    {
        Self(Box::new(f))
    }

    pub fn execute(self, ctx: Context<Matter>) {
        let f = self.0;
        f(ctx);
    }
}
