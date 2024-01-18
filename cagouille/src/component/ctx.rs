use std::cell::RefCell;

use futures::{AsyncWrite, AsyncWriteExt, future::LocalBoxFuture};

use crate::vdom::{VNodeScope, traits::{Renderable, RenderToStream}, RenderMode, VNode};

use super::traits::Component;

/// Render context of a component
pub struct Context<'node, Comp> where Comp: Component {    
    /// State of the component
    pub data: &'node Comp::Data,
    /// Scope of the component
    pub scope: &'node VNodeScope,
    /// Events
    pub events: &'node Comp::Events
}


