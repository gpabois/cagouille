use std::any::Any;

use crate::Context;

pub struct AnyAction(pub(super) Box<dyn Any + Sync + Send + 'static>);
pub type DynActionFn<Matter> = dyn FnOnce(Context<Matter>) + Sync + Send + 'static;

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

/// An action in the reactor.
pub struct Action<Matter>(Box<DynActionFn<Matter>>);

impl<Matter> Action<Matter> {
    pub fn execute(self, ctx: Context<Matter>) {
        let f = self.0;
        f(ctx);
    }
}

impl<Matter> Action<Matter> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Context<Matter>) + Sync + Send + 'static,
    {
        Self(Box::new(f))
    }
}

pub mod local {
    use std::any::Any;

    use crate::Context;

    pub type DynLocalActionFn<Matter> = dyn FnOnce(Context<Matter>) + 'static;

    pub struct LocalAnyAction(pub(super) Box<dyn Any>);

    /// An action, without the Sync + Send constraint relaxed.
    pub struct LocalAction<Matter>(Box<DynLocalActionFn<Matter>>);

    impl<Matter> LocalAction<Matter> {
        pub fn execute(self, ctx: Context<Matter>) {
            let f = self.0;
            f(ctx);
        }
    }

    impl<Matter> LocalAction<Matter> {
        pub fn new<F>(f: F) -> Self
        where
            F: FnOnce(Context<Matter>) + 'static,
        {
            Self(Box::new(f))
        }
    }
}
