use crate::vdom::{VDomNodeScope, traits::Renderable, RenderMode};

use super::traits::Component;


/// State of a component
pub struct State<Comp> where Comp: Component {
    pub scope:      VDomNodeScope,
    pub props:      Comp::Properties,
    pub data:       Comp::Data,
}

impl<Comp> State<Comp> 
where Comp: Component
{
    /// Create a new root component state
    pub async fn new_root(mode: RenderMode, props: Comp::Properties) -> Self {
        let data = Comp::data(&props).await;
        Self{scope: VDomNodeScope::new_root(mode), data, props}        
    }
    /// Create a new component state
    pub async fn new(parent: &VDomNodeScope, props: Comp::Properties) -> Self {
        let data = Comp::data(&props).await;
        Self{scope: parent.new_child_scope(), data, props}
    }
}

impl<'a, Comp> Renderable<'a> for &'a State<Comp> where Comp: Component {
    fn render<'fut>(self) -> futures::prelude::future::LocalBoxFuture<'fut, Result<crate::vdom::VNode, crate::error::Error>> where 'a: 'fut {
        Comp::render(self)
    }
}


