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
use tracker::Tracker;
use action::{Action, AnyAction};
use reaction::{Reaction, AnyReaction};
use interaction::{Interaction, AnyInteraction, BoundInteraction};
use interface::{Signal, SignalRx, Slot, SlotTx};

pub use atom::Atom;
pub use ray::Ray;
pub use measure::Measure;
pub use context::{Context, InitContext};

use crate::executor::LocalExecutor;

pub struct Reactor<Matter> 
where Matter: 'static
{
    _pht: std::marker::PhantomData<Matter>,
    signal: Signal,
    slot: Slot
}

impl<Matter> Reactor<Matter> 
    where Matter: 'static
{
    /// Create a new reactor core, and returns its pilot
    pub fn new<Spawner, F>(init: F) -> Self 
    where 
        Spawner: yase::LocalSpawner,
        F: FnOnce(InitContext<Matter>) -> Matter + 'static
    {
        let (signal, signal_rx) = Signal::create();
        let (slot, slot_tx) = Slot::create();

        let sig2 = signal.clone();
        let slot2 = slot.clone();

        let join = Spawner::spawn(async move {
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

    /// Create a new reactor core, and returns its pilot
    pub fn new_async<E, F, Fut>(init: F) -> Self 
    where 
        E: LocalExecutor,
        F: FnOnce(InitContext<Matter>) -> Fut + 'static,
        Fut: Future<Output=Matter> + 'static
    {
        let (signal, signal_rx) = Signal::create();
        let (slot, slot_tx) = Slot::create();

        let sig2 = signal.clone();
        let slot2 = slot.clone();
        // The core lives within a future.
        let join = E::spawn_local(async move {
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
}
