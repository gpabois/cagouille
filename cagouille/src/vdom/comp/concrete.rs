use std::fmt::Debug;

use futures::future::LocalBoxFuture;

use crate::component::traits::Component;
use crate::component::State;

use crate::error::Error;
use crate::vdom::VNode;

use super::driver::CompDriver;

pub enum ConcreteComponentNode<Comp>
where
    Comp: Component + 'static,
{
    Uninitialised {
        props: Comp::Properties,
        events: Comp::Events,
    },
    Initialised {
        state: State<Comp>,
        vnode: VNode,
    },
}

impl<Comp> Debug for ConcreteComponentNode<Comp>
where
    Comp: Component,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uninitialised { props, events } => f
                .debug_struct("Uninitialised")
                .field("props", props)
                .field("events", events)
                .finish(),
            Self::Initialised { state, vnode: _ } => {
                f.debug_struct("Initialised").field("state", state).finish()
            }
        }
    }
}

impl<Comp> ConcreteComponentNode<Comp>
where
    Comp: Component + 'static,
{
    pub fn new(props: Comp::Properties, events: Comp::Events) -> Self {
        Self::Uninitialised { props, events }
    }

    pub fn is_initialised(&self) -> bool {
        match self {
            Self::Initialised { state: _, vnode: _ } => true,
            _ => false,
        }
    }
}

impl<Comp> CompDriver for ConcreteComponentNode<Comp>
where
    Comp: Component + 'static,
{
    /// Initialise the component.
    /// Call render once.
    fn initialise<'fut>(&'fut mut self) -> LocalBoxFuture<'fut, Result<(), Error>> {
        if self.is_initialised() {
            return Box::pin(std::future::ready(Ok(())));
        }

        Box::pin(async {
            match self {
                Self::Uninitialised { props, events } => {
                    let props = std::mem::take(props);
                    let events = std::mem::take(events);

                    let state = State::new(props, events);
                    let vnode = state.vnode.changed().await.take();

                    *self = Self::Initialised { state, vnode };

                    Ok(())
                }
                _ => Ok(()),
            }
        })
    }
}
