use self::attr::{ElementAttribute, ElementAttributes};
use futures::{future::LocalBoxFuture, AsyncWrite, AsyncWriteExt};

use super::{traits::RenderToStream, VNode};

pub mod attr;
pub mod df;
pub mod event;

#[derive(Clone)]
/// Virtual HTML element
pub struct ElementNode {
    pub(super) tag: String,
    pub(super) attributes: ElementAttributes,
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
        IntoAttrs: IntoIterator<Item = ElementAttribute>,
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

impl<'a> RenderToStream<'a> for &'a ElementNode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(
        self,
        stream: &'stream mut W,
    ) -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where
        'a: 'fut,
        'stream: 'fut,
    {
        Box::pin(async {
            stream.write_all("<".as_bytes()).await?;
            stream.write_all(self.tag.as_bytes()).await?;
            self.attributes.render_to_stream(stream).await?;
            stream.write_all(">".as_bytes()).await?;

            for child in self.children.iter() {
                child.render_to_stream(stream).await?;
            }

            stream.write_all("</".as_bytes()).await?;
            stream.write_all(self.tag.as_bytes()).await?;
            stream.write_all(">".as_bytes()).await?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::vdom::{traits::RenderToStream, Scope, VNode};

    #[tokio::test]
    async fn test_render_to_stream() {
        let root_scope = Scope::new_root();
        let el: VNode = VNode::element(
            root_scope.new_child_scope(),
            "div",
            [("class", "px-2")],
            [VNode::element(root_scope.new_child_scope(), "p", [], [])],
        );

        let str = el.render_to_string().await.unwrap();

        assert_eq!(str, "<div class=\"px-2\"><p></p></div>");
    }
}
