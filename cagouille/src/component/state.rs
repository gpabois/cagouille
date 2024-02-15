use reactor::Reactor;

use crate::vdom::{mode::Mode, VNode};

use super::traits::Component;

pub struct Matter<M, C>
where
    C: Component<M>,
    M: Mode,
{
    data: C::Data,
    events: C::Events,
    props: C::Properties,
}

/// State of a component.
pub struct State<M, C>
where
    C: Component<M>,
    M: Mode,
{
    reactor: Reactor<Matter<M, C>>,
    vnode: reactor::Measure<VNode<M>>,
}

impl<M, C> State<M, C>
where
    C: Component<M>,
    M: Mode,
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
        ctx: reactor::InitContext<Matter<M, C>>,
    ) -> Matter<M, C> {
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
