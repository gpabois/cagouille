use std::ops::{Deref, DerefMut};
use crate::sync::{
    Atom,
    Ray,
    Tracker,
    Signal,
    Slot,
    Reaction
};


/// Initialisation context, to create the matter
pub struct InitContext<Matter>
where
    Matter: Sync + Send + 'static,
{
    _pht: std::marker::PhantomData<Matter>,
    signal: Signal,
    slot: Slot,
}

impl<Matter> InitContext<Matter>
where
    Matter: Sync + Send + 'static,
{
    /// Create a new init context.
    pub fn new(signal: Signal, slot: Slot) -> Self {
        Self {
            signal,
            slot,
            _pht: Default::default(),
        }
    }

    /// Creates a new atom
    pub fn use_atom<D>(&self, value: D) -> Atom<D> {
        Atom::new(value, Tracker::new(self.slot.clone()))
    }

    /// Creates a new interaction
    pub fn use_interaction<F>(&self, f: F) 
    where F: Fn(Context<Matter>) + Sync + Send + 'static
    {
        self.signal.send(Reaction::interact(f))
    }

    /// Creates a new ray
    pub fn use_ray<D, F>(&self, init: D, f: F) -> Ray<D>
    where
        F: Fn(Context<Matter>) -> D + Sync + Send + 'static,
        D: Sync + Send + 'static,
    {
        Ray::new(
            init,
            f,
            self.signal.clone(),
            Tracker::new(self.slot.clone()),
        )
    }
}

/// The context of a reaction (reactor's command)
pub struct Context<'ctx, Matter> {
    matter: &'ctx mut Matter,
}

impl<'ctx, Matter> Context<'ctx, Matter> {
    pub fn new(matter: &'ctx mut Matter) -> Self {
        Self { matter }
    }
}

impl<'ctx, Matter> Deref for Context<'ctx, Matter> {
    type Target = Matter;

    fn deref(&self) -> &Self::Target {
        self.matter
    }
}

impl<'ctx, Matter> DerefMut for Context<'ctx, Matter> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.matter
    }
}
