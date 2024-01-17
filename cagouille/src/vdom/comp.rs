use crate::component::{traits::Component, state::State};

use super::VDomNodeScope;

pub struct ComponentNode<Comp: Component> {
    state: State<Comp>
}

impl<Comp> ComponentNode<Comp> where Comp: Component {
    pub async fn new(parent: &VDomNodeScope, props: Comp::Properties) -> Self {
        Self {
            state: State::new(parent, props).await
        }
    }
}