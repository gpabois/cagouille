pub mod component;
pub mod df;
pub mod dom;
pub mod error;
pub mod event;
pub mod futures;
pub mod vdom;

pub mod prelude {
    pub use cagouille_macro::component;
    pub use cagouille_macro::render;
    pub use cagouille_macro::Differentiable;
}

pub mod helpers {
    use crate::component::traits::Component;
    use crate::vdom::el::Attribute;
    use crate::vdom::{Scope, VNode};

    /// Create a new element attribute
    pub fn a<IntoName, IntoValue>(name: IntoName, value: IntoValue) -> Attribute {
        Attribute::new(name.into(), value.into())
    }
    
    /// Create a new virtual element.
    pub fn e<IntoTag, IntoAttrs, IntoChildren, ChildrenScopeFn>(scope: Scope, tag: IntoTag, attrs: IntoAttrs, children: ChildrenScopeFn) -> VNode 
    where
        IntoTag: Into<String>,
        IntoAttrs: IntoIterator<Item = Attribute>,
        IntoChildren: IntoIterator<Item = VNode>,
        ChildrenScopeFn: FnOnce(&Scope) -> IntoChildren
    {
        let children = children(&scope);

        VNode::element(scope, tag, attrs, children)
    }

    /// Create a new component
    pub fn c<Comp>(scope: Scope, props: Comp::Properties, events: Comp::Events) where Comp: Component {
        VNode::component::<Comp>(scope, props, events)
    }
}