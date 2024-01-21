use futures::{future::{join_all, LocalBoxFuture, BoxFuture}, Future};
use tokio::join;

use crate::component::traits::Component;

use super::{VNode, traits::RenderToStream, comp::ComponentNode};

/// Mounted within a DOM tree.
pub struct BrowserMode;

/// Serialize the virtual dom into a string.
pub struct DebugMode;

impl Mode for DebugMode {
    type ComponentNodeState = ();

    /// We recursively initialise the whole tree
    fn on_component_node_initialised<'a, 'fut, Comp: Component<Self>>(node: &'a ComponentNode<Self, Comp>) -> LocalBoxFuture<'fut, ()> 
    where 'a: 'fut
    {
        Box::pin(async {
            // Initialise rendering 
            let v_node = node.state.render().await.expect("cannot render the component");
            v_node.initialise().await;
            *node.v_node.0.write().await = Some(v_node);
        })
    }
}

impl DebugMode {
    pub async fn render_to_string<Comp: Component<Self> + 'static>(props: Comp::Properties, events: Comp::Events) -> String {
        let root_scope = super::Scope::new_root();
        let root: VNode<_> = VNode::<Self>::component::<Comp>(&root_scope, props, events).into();
        root.initialise().await;
        root.render_to_string().await.expect("cannot render to string")
    }
}



/// SSR rendering.
pub struct SSRMode;

pub trait Mode: Sized + 'static {
    /// Data stored in the component node state for the mode handler.
    type ComponentNodeState: Default;

    /// What to do once a component node has been initialised.
    fn on_component_node_initialised<'a, 'fut, Comp: Component<Self>>(comp: &'a ComponentNode<Self, Comp>) -> LocalBoxFuture<'fut, ()> where 'a: 'fut;
}
