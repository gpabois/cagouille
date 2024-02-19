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
            Self::Initialised { state } => {
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
            Self::Initialised { state: _ } => true,
            _ => false,
        }
    }

    pub fn get_state(&self) -> Option<&State<Comp>> {
        match &self {
            ConcreteComponentNode::Uninitialised {
                props: _,
                events: _,
            } => None,
            ConcreteComponentNode::Initialised { state } => Some(state),
        }
    }
}

impl<Comp> CompDriver for ConcreteComponentNode<Comp>
where
    Comp: Component + 'static,
{
    /// Initialise the component
    fn initialise<'a, 'fut>(&'a mut self) -> LocalBoxFuture<'fut, Result<(), Error>>
    where
        'a: 'fut,
    {
        if self.is_initialised() {
            return Box::pin(std::future::ready(Ok(())));
        }

        match self {
            Self::Uninitialised { props, events } => {
                let props = std::mem::take(props);
                let events = std::mem::take(events);

                *self = Self::Initialised {
                    state: State::new(props, events),
                };
            }
            _ => {}
        };

        let mut vnode_measure = self.get_state().unwrap().vnode.clone();
        let (init_tx, init_rx) = tokio::sync::oneshot::channel::<Result<(), Error>>();

        // Spawn the virtual node watcher
        tokio::spawn(async move {
            vnode_measure.changed().await;

            let mut vnode: VNode = vnode_measure.take();
            vnode.initialise().await;
            init_tx.send(Ok(()));

            loop {
                vnode_measure.changed().await;
                let new_vnode = vnode_measure.take();
                // Patch
                vnode.patch(new_vnode);
            }
        });

        // Initialise component's root vnode.
        Box::pin(async move { init_rx.await.unwrap() })
    }
}
