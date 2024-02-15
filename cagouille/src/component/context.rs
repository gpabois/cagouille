use crate::vdom::mode::Mode;

use super::state::Matter;
use super::traits::Component;

/// Initialisation context of the component
pub struct InitContext<'a, M, C>
where
    C: Component<M> + 'static,
    M: Mode,
{
    pub props: &'a C::Properties,
    pub hooks: reactor::InitContext<Matter<M, C>>,
}

pub type Context<'a, M, C> = reactor::Context<'a, Matter<M, C>>;
