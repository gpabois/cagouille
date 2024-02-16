use reactor::Reactor;

use crate::vdom::VNode;

use super::{context::InitContext, traits::Component};

pub struct Matter<C>
where
    C: Component
{
    data: C::Data,
    events: C::Events,
    props: C::Properties,
}

/// State of a component.
pub struct State<C>
where
    C: Component,
{
    reactor: Reactor<Matter<C>>,
    vnode: reactor::Measure<VNode>,
}

impl<C> State<C>
where
    C: Component,
{
    pub fn new(props: C::Properties, events: C::Events) -> Self {
        let mut reactor = Reactor::new(move |ctx| Self::initialise(props, events, ctx));

        // Rerender each time the matter is updated.
        let vnode = reactor.use_measure(VNode::Empty, |ctx| C::render(ctx).unwrap());

        Self { reactor, vnode }
    }

    /// Bridge between reactor lifecycle and component lifecycle
    fn initialise(
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
