use std::{sync::{Arc, Weak}, ops::DerefMut};

use async_std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::vdom::{Scope, VNode, mode::Mode};

use super::{traits::Component, ctx::{MutContext, Context}};

struct Inner<M, Comp> 
where Comp: Component<M>, M: Mode {
    scope:  Scope,
    props:  Comp::Properties,
    data:   Option<Comp::Data>,
    events: Comp::Events,
}

impl<M, Comp> Default for Inner<M, Comp> where Comp: Component<M>, M: Mode {
    fn default() -> Self {
        Self { scope: Default::default(), props: Default::default(), data: Default::default(), events: Default::default() }
    }
}

/// A weak reference to a component state.
/// Does not own any ownership on the state, so it could have been dropped.
pub struct WeakStateRef<M, Comp>(Weak<RwLock<Inner<M, Comp>>>) where Comp: Component<M>, M: Mode;

impl<M, Comp> WeakStateRef<M, Comp> where Comp: Component<M>, M: Mode {
    /// Attempts to upgrade the Weak pointer to a State, delaying dropping of the inner value if successful.
    pub fn upgrade(self) -> Option<State<M, Comp>> {
        match self.0.upgrade() {
            Some(arc) => Some(State(arc)),
            None => None
        }
    }
}

impl<M, Comp> Clone for WeakStateRef<M, Comp> where Comp: Component<M>, M: Mode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// Borrowed state ref.
pub struct StateRef<'a, M, Comp>(RwLockReadGuard<'a, Inner<M, Comp>>) where Comp: Component<M>, M: Mode;

impl<'a, M, Comp> StateRef<'a, M, Comp> where Comp: Component<M>, M: Mode {
    pub fn to_context(&'a self, weak: WeakStateRef<M, Comp>) -> Context<'a, M, Comp> {
        Context {
            weak,
            scope:  &self.0.scope,
            data:   self.0.data.as_ref().expect("not initialised"),
            events: &self.0.events
        }
    }
}

/// Mut-borrowed state ref.
pub struct StateMut<'a, M, Comp>(RwLockWriteGuard<'a, Inner<M, Comp>>) where Comp: Component<M>, M: Mode;

impl<'a, M, Comp> StateMut<'a, M, Comp> where Comp: Component<M>, M: Mode 
{
    pub fn to_mut_context<'b>(&'b mut self, weak: WeakStateRef<M, Comp>) -> MutContext<'b, M, Comp> where 'a: 'b {
        let rf = self.0.deref_mut() as *mut Inner<M, Comp>;
        
        // Hack the borrow-checker
        // Otherwise cannot immutable borrow scope, and events while mut borrowing data...
        // This should not lead to any use-after-free as we enforce lifetime constraints such as :
        // - the mut context is not movable.
        // - the mut context lives as long as the state mut reference 
        // - the write guard must live at least as long as the state mut reference
        unsafe {
            MutContext {
                weak,
                scope: &(*rf).scope,
                events: &(*rf).events,
                data: (&mut *rf).data.as_mut().expect("not initialised"),
            }
        }
    }
}

pub struct State<M, Comp>(Arc<RwLock<Inner<M, Comp>>>) where Comp: Component<M>, M: Mode;

impl<M, Comp> Default for State<M, Comp> where Comp: Component<M>, M: Mode {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<M, Comp> State<M, Comp> where Comp: Component<M>, M: Mode {
    pub fn new(scope: Scope, props: Comp::Properties, events: Comp::Events) -> Self {
        Self(
            Arc::new(RwLock::new(
                Inner { 
                    scope:  scope, 
                    props:  props, 
                    data:   None, 
                    events: events 
                }
            ))
        )
    }

    /// Initialise the data of the component.
    pub async fn initialise(&self) {
        {
            let mut mut_state = self.0.write().await;
            mut_state.data = Some(Comp::data(&mut_state.props).await);
        }

        let mut borrowed = self.borrow_mut().await;
        let ctx = borrowed.to_mut_context(self.weak());
        Comp::initialised(ctx).await;
    }

    /// Render the component
    pub async fn render(&self) -> Result<VNode<M>, crate::error::Error> {
        let weak = self.weak();
        let borrowed = self.borrow().await;
        let ctx = borrowed.to_context(weak);
        
        Comp::render(ctx).await
    }

    /// Creates a weak reference to the component state
    pub fn weak(&self) -> WeakStateRef<M, Comp> {
        WeakStateRef(Arc::downgrade(&self.0))
    }

    /// Borrow the state
    pub async fn borrow(&self) -> StateRef<'_, M, Comp> {
        let state_ref = self.0.read().await;
        StateRef(state_ref)
    }

    /// Mutably borrow the state
    pub async fn borrow_mut(&self) -> StateMut<'_, M, Comp> {
        let state_mut_ref = self.0.write().await;
        StateMut(state_mut_ref)
    }
}