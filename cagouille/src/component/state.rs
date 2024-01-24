use std::{any::Any, ops::DerefMut, sync::{Arc, Weak}};

use async_std::{sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}, task};
use futures::future::LocalBoxFuture;
use tokio::pin;

use crate::{df::traits::{AsyncDifferentiable, Differentiable, Df}, vdom::{Scope, VNode, mode::Mode}};

use super::{traits::Component, ctx::{MutContext, Context}};

/// Differential of a component state
pub struct StateDf<M, Comp> where Comp: Component<M>, M: Mode {
    // Differential of properties
    props_df: <Comp::Properties as Differentiable>::Df
}

impl<M, Comp> Df<State<M, Comp>> for StateDf<M, Comp> where M: Mode, Comp: Component<M> + 'static {
    fn apply(self, dest: &mut State<M, Comp>) {
        let weak = dest.weak();

        // Schedule a patch of the component.
        tokio::task::spawn_local(async move {
            let rf = weak.upgrade();

            if rf.is_none() {
                return;
            }

            rf.unwrap().patch(self).await
        });
    }
}

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

impl<M, Comp> Clone for State<M, Comp> where Comp: Component<M>, M: Mode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M, Comp> AsyncDifferentiable for State<M, Comp> where M: Mode, Comp: Component<M> + 'static {
    type Df = StateDf<M, Comp>;

    fn df<'a, 'fut>(src: &'a Self, dest: &'a Self) -> LocalBoxFuture<'fut, Self::Df> where 'a: 'fut {
        let fut = async {
            let rsrc = src.0.read().await;
            let rdest = dest.0.read().await;

            StateDf {
                props_df: <Comp::Properties as Differentiable>::df(&rsrc.props, &rdest.props)
            }
        };

        Box::pin(fut)
    }
}

impl<M, Comp> State<M, Comp> where Comp: Component<M> + 'static, M: Mode {
    /// Patch the state of the component
    pub async fn patch(&mut self, ds: StateDf<M, Comp>) {
        ds.apply(self)
    }
}

pub struct AnyState(Box<dyn Any>);

impl AnyState {
    pub fn downcast<M, Comp>(self) -> Option<State<M, Comp>> where M: Mode, Comp: Component<M> + 'static {
        self.0.downcast::<State<M,Comp>>().ok().map(|b| *b)
    }
}

impl<M, Comp> State<M, Comp> where Comp: Component<M> + 'static, M: Mode {
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

    /// Erase the state's concrete type.
    pub fn to_any(&self) -> AnyState {
        AnyState(Box::new(self.clone()))
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