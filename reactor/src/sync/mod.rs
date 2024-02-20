mod reaction;
mod action;
mod context;
mod interface;
mod interaction;
mod atom;
mod ray;
mod tracker;
mod measure;
mod core;

use core::Core;
use std::future::Future;
use tokio::task::JoinHandle;
use tracker::Tracker;
use action::{Action, AnyAction};
use reaction::{Reaction, AnyReaction};
use interaction::{Interaction, AnyInteraction, BoundInteraction};
use interface::{Signal, SignalRx, Slot, SlotTx};

pub use atom::Atom;
pub use ray::Ray;
pub use measure::Measure;
pub use context::{Context, InitContext};

use crate::executor::Executor;

pub struct Reactor<Matter> 
where Matter: Sync + Send + 'static
{
    _pht: std::marker::PhantomData<Matter>,
    signal: Signal,
    slot: Slot
}

impl<Matter> Reactor<Matter> 
where Matter: Sync + Send + 'static
{
    pub fn act<F>(&self, f: F) 
    where
        F: FnOnce(Context<Matter>) + Sync + Send + 'static
    {
        self.signal.send(Action::new(f))
    }

    /// Creates a new measure and wait for a value to be set.
    pub fn use_measure<D, F>(&self, f: F) -> Measure<D>
    where 
        D: Default + Sync + Send + 'static,
        F: Fn(Context<'_, Matter>) -> D + Sync + Send + 'static
    {
        Measure::new(D::default(), f, self.signal.clone())
    }

    /// Creates a new measure and wait for a value to be set.
    pub async fn use_stabilised_measure<D, F>(&self, f: F) -> Measure<D>
    where 
        D: Default + Sync + Send + 'static,
        F: Fn(Context<'_, Matter>) -> D + Sync + Send + 'static
    {
        let mut measure = Measure::new(D::default(), f, self.signal.clone());
        measure.changed().await;
        measure
    }
}

impl<Matter> Reactor<Matter> 
where Matter: Sync + Send + 'static
{
    pub fn new_async<E, F, Fut>(init: F) -> Self
    where 
        E: Executor,
        F: FnOnce(InitContext<Matter>) -> Fut + Sync + Send + 'static,
        Fut: Future<Output=Matter> + Sync + Send + 'static
    {
        let (signal, signal_rx) = Signal::create();
        let (slot, slot_tx) = Slot::create();

        let sig2 = signal.clone();
        let slot2 = slot.clone();
        // The core lives within a future.
        let join = E::spawn(async move {
            let init_ctx = InitContext::new(sig2.clone(), slot2);

            let core = Core::new(
                init(init_ctx).await,
                sig2,
                signal_rx,
                slot_tx
            );

            core.r#loop().await;
        });

        Reactor {
            signal,
            slot,
            _pht: Default::default()
        }
    }
    /// Create a new reactor core, and returns its pilot
    pub fn new<E, F>(init: F) -> Self
    where 
        E: Executor,
        F: FnOnce(InitContext<Matter>) -> Matter + Sync + Send + 'static
    {
        let (signal, signal_rx) = Signal::create();
        let (slot, slot_tx) = Slot::create();

        let sig2 = signal.clone();
        let slot2 = slot.clone();
        // The core lives within a future.
        let join = E::spawn(async move {
            let init_ctx = InitContext::new(sig2.clone(), slot2);

            let core = Core::new(
                init(init_ctx),
                sig2,
                signal_rx,
                slot_tx
            );

            core.r#loop().await;
        });

        Reactor {
            signal,
            slot,
            _pht: Default::default()
        }
    }
}