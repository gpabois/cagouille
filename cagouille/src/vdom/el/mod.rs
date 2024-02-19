pub use self::attr::{Attribute, Attributes};
use super::VNode;

mod render;
pub mod attr;
pub mod df;
pub mod event;

#[derive(Clone)]
/// Virtual HTML element
pub struct ElementNode {
    pub(super) tag: String,
    pub(super) attributes: Attributes,
    pub(super) children: Vec<VNode>,
}

impl Default for ElementNode {
    fn default() -> Self {
        Self {
            tag: Default::default(),
            attributes: Default::default(),
            children: Default::default(),
        }
    }
}

impl ElementNode {
    pub fn new<IntoTag, IntoAttrs, IntoChildren>(
        tag: IntoTag,        
        attrs: IntoAttrs,
        children: IntoChildren,
    ) -> Self
    where
        IntoTag: Into<String>,
        IntoAttrs: IntoIterator<Item = Attribute>,
        IntoChildren: IntoIterator<Item = VNode>,
    {
        Self {
            tag: tag.into(),
            attributes: attrs.into_iter().collect(),
            children: children.into_iter().collect(),
        }
    }

    pub async fn initialise(&mut self) {
        futures::future::join_all(self.children.iter_mut().map(VNode::initialise)).await;
    }
}


#[cfg(test)]
mod tests {
    use crate::vdom::{traits::RenderToStream, Scope};
    use crate::helpers::{a, e};
    #[tokio::test]
    async fn test_render_to_stream() {
        let root = Scope::new_root();

        let el = e(root.child(), "div", [a("class", "px-2")], |scope| [
            e(scope.child(), "p", [], |_| [])
        ]);
        let str = el.render_to_string().await.unwrap();

        assert_eq!(str, "<div class=\"px-2\"><p></p></div>");
    }
}
