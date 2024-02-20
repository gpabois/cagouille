use crate::{interface::Signal, Context};

use std::{
    any::Any,
    sync::{Arc, RwLock},
};

struct BoundInteractionInner {
    interaction: AnyInteraction,
    signal: Signal,
    /// The bound interaction is scheduled to be executed
    scheduled: RwLock<bool>,
}

impl PartialEq for BoundInteractionInner {
    fn eq(&self, other: &Self) -> bool {
        self.interaction == other.interaction && self.signal == other.signal
    }
}

#[derive(Clone, PartialEq)]
/// Interaction bound to a reactor
pub(crate) struct BoundInteraction(Arc<BoundInteractionInner>);

impl BoundInteraction {
    ///
    pub fn new(interaction: AnyInteraction, signal: Signal) -> Self {
        Self(Arc::new(BoundInteractionInner {
            interaction,
            signal,
            scheduled: RwLock::new(false),
        }))
    }

    /// Send interaction to the reactor
    pub fn schedule(&self) {
        if !*self.0.scheduled.read().unwrap() {
            *self.0.scheduled.write().unwrap() = true;
            self.0.signal.send(self.clone());
        }
    }

    pub fn ack(&self) {
        *self.0.scheduled.write().unwrap() = false;
    }

    pub fn downcast<Matter>(&self) -> Option<Interaction<Matter>>
    where
        Matter: 'static,
    {
        self.0.interaction.clone().downcast()
    }
}

#[derive(Clone)]
/// Any interaction
pub struct AnyInteraction(Arc<dyn Any + Sync + Send + 'static>);

impl<Matter> From<Interaction<Matter>> for AnyInteraction
where
    Matter: 'static,
{
    fn from(value: Interaction<Matter>) -> Self {
        Self(value.0)
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

impl<Matter> Clone for Interaction<Matter> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Matter> Interaction<Matter> {
    /// Create a new interaction
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Context<'_, Matter>) + Sync + Send + 'static,
    {
        Self(Arc::new(InteractionInner(Box::new(f))))
    }

    /// Execute the interaction
    pub fn execute(&self, context: Context<'_, Matter>) {
        self.0 .0(context);
    }
}

pub mod local {
    use std::{any::Any, rc::Rc};

    use crate::Context;

    type LocalInteractionFn<Matter> = dyn Fn(Context<'_, Matter>) + 'static;

    struct LocalInteractionInner<Matter>(Box<LocalInteractionFn<Matter>>);

    /// An interaction is a function run by the reactor's core
    pub struct LocalInteraction<Matter>(Rc<LocalInteractionInner<Matter>>);

    impl<Matter> PartialEq for LocalInteraction<Matter> {
        fn eq(&self, other: &Self) -> bool {
            Rc::as_ptr(&self.0) == Rc::as_ptr(&other.0)
        }
    }

    impl<Matter> Clone for LocalInteraction<Matter> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<Matter> LocalInteraction<Matter> {
        /// Create a new interaction
        pub fn new<F>(f: F) -> Self
        where
            F: Fn(Context<'_, Matter>) + 'static,
        {
            Self(Rc::new(LocalInteractionInner(Box::new(f))))
        }

        /// Execute the interaction
        pub fn execute(&self, context: Context<'_, Matter>) {
            self.0 .0(context);
        }
    }

    #[derive(Clone)]
    /// Any interaction
    pub struct LocalAnyInteraction(Rc<dyn Any + 'static>);

    impl<Matter> From<LocalInteraction<Matter>> for LocalAnyInteraction
    where
        Matter: 'static,
    {
        fn from(value: LocalInteraction<Matter>) -> Self {
            Self(value.0)
        }
    }

    impl PartialEq for LocalAnyInteraction {
        fn eq(&self, other: &Self) -> bool {
            std::ptr::addr_eq(Rc::as_ptr(&self.0), Rc::as_ptr(&other.0))
        }
    }

    impl LocalAnyInteraction {
        pub fn downcast<Matter>(self) -> Option<LocalInteraction<Matter>>
        where
            Matter: 'static,
        {
            match self.0.downcast::<LocalInteractionInner<Matter>>() {
                Ok(inner) => Some(LocalInteraction(inner)),
                Err(_) => None,
            }
        }
    }
}
