use futures::{future::LocalBoxFuture, Future};

use crate::vdom::{Scope, mode::Mode};

use super::{traits::Component, state::WeakStateRef, event::traits::{Event, EventSignal}};

/// Read-only component context
pub struct Context<'ctx, M, Comp> where Comp: Component<M>, M: Mode {    
    /// State of the component
    pub data: &'ctx Comp::Data,
    
    /// Scope of the component
    pub scope: &'ctx Scope,
    
    /// Events
    pub events: &'ctx Comp::Events,
    
    /// Weak ref
    pub(super) weak:  WeakStateRef<M, Comp>
}

pub struct MutContext<'ctx, M, Comp> where Comp: Component<M>, M: Mode {
    /// State of the component
    pub data: &'ctx mut Comp::Data,
    
    /// Scope of the component
    pub scope: &'ctx Scope,
    
    /// Events
    pub events: &'ctx Comp::Events,

    /// Weak ref
    pub(super) weak:  WeakStateRef<M, Comp>
}

impl<'ctx, M, Comp> Context<'ctx, M, Comp> where Comp: Component<M> + 'static, M: Mode {
    /// Emit an event
    pub fn emit<E: Event>(&self, _: E, payload: E::Payload) where for<'a> Comp::Events: EventSignal<'a, E> {
        self.events.emit(payload)
    }

    /// Perform a state mutation
    pub fn mutate<F, Fut: Future>(&self, mutator: F) 
    where F: for <'a> FnOnce(MutContext<'a, M, Comp>) -> Fut + 'static
    {
        let weak = self.weak.clone();
        
        tokio::task::spawn_local(async move {
            let upgrade_result = weak.clone().upgrade();
            if upgrade_result.is_none() { return; }

            let state = upgrade_result.unwrap();
            let mut mut_state = state.borrow_mut().await;
            let mut_ctx = mut_state.to_mut_context(weak);
            mutator(mut_ctx).await;
        });
    }
}

