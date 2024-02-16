use std::any::TypeId;

use futures::future::LocalBoxFuture;

use crate::component::traits::Component;
use crate::component::State;

use crate::df::traits::Differentiable;
use crate::error::Error;
use crate::vdom::Mode;
use crate::vdom::VNode;

use super::driver::CompDriver;
use super::ComponentNode;

pub enum ConcreteComponentNode<Comp>
where
    Comp: Component,
{
    Uninitialised {
        props: Comp::Properties,
        events: Comp::Events,
    },
    Initialised {
        state: State<Comp>,
    },
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
            Self::Initialised { state } => true,
            _ => false,
        }
    }
}

impl<Comp> CompDriver for ConcreteComponentNode<Comp>
where
    Comp: Component + 'static,
    Comp::Properties: Differentiable,
{
    /// Initialise the component
    fn initialise<'a, 'fut>(&'a mut self) -> LocalBoxFuture<'fut, Result<(), Error>>
    where
        'a: 'fut,
    {
        if self.is_initialised() {
            return std::future::ready(Ok(()));
        }

        Box::pin(async {
            match self {
                Self::Uninitialised { props, events } => {
                    let props = std::mem::take(props);
                    let events = std::mem::take(events);

                    *self = Self::Initialised { state: State::new(props, events) };

                },
                _ => {}
            }

            Ok(())
        })
    }
}

impl<Comp> Into<ComponentNode> for ConcreteComponentNode<Comp>
where
    Comp: Component + 'static,
    Comp::Properties: Differentiable,
{
    fn into(self) -> ComponentNode {
        ComponentNode {
            driver: Box::new(self),
        }
    }
}

impl<Comp> Into<VNode> for ConcreteComponentNode<Comp>
where
    Comp: Component + 'static,
    Comp::Properties: Differentiable,
{
    fn into(self) -> VNode {
        VNode::Component(self.into())
    }
}
