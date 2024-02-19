use crate::vdom::{Scope, VNode};
use reactor::Reactor;
use std::fmt::Debug;

use super::{context::InitContext, traits::Component};

#[derive(Debug)]
pub struct Matter<C>
where
    C: Component,
{
    data: C::Data,
    events: C::Events,
    props: C::Properties,
}

/// State of a component.
pub struct State<C>
where
    C: Component + 'static,
{
    reactor: Reactor<Matter<C>>,
    pub(crate) vnode: reactor::Measure<VNode>,
}

impl<C> Debug for State<C>
where
    C: Component,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish()
    }
}

impl<C> State<C>
where
    C: Component + 'static,
{
    pub fn new(props: C::Properties, events: C::Events) -> Self {
        let reactor = Reactor::async_new(move |ctx| Box::pin(Self::initialise(props, events, ctx)));

        // Rerender each time the matter is updated.
        let vnode = reactor.use_measure(VNode::empty(Scope::default()), |ctx| {
            C::render(ctx).unwrap()
        });

        Self { reactor, vnode }
    }

    /// Bridge between reactor lifecycle and component lifecycle
    async fn initialise(
        props: C::Properties,
        events: C::Events,
        ctx: reactor::InitContext<Matter<C>>,
    ) -> Matter<C> {
        let data = {
            let ctx = InitContext {
                props: &props,
                hooks: ctx,
            };
            C::initialise(ctx)
        };

        Matter {
            data,
            events,
            props,
        }
    }
}
