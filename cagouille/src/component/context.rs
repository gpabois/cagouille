use super::state::Matter;
use super::traits::Component;

/// Initialisation context of the component
pub struct InitContext<'a, C>
where
    C: Component + 'static,
{
    pub props: &'a C::Properties,
    pub hooks: reactor::InitContext<Matter<C>>,
}

pub type Context<'a, C> = reactor::Context<'a, Matter<C>>;
