use super::state::Matter;


/// Initialisation context of the component
pub struct InitContext<'a, M,C> where C: Component<M>{
    pub props: &C::Properties,
    pub hooks: InitContext<Matter<M,C>>
}

pub type Context<'a, M, C> = reactor::Context<'a, Matter<M,C>>;
