use futures::{AsyncWriteExt, AsyncWrite, future::LocalBoxFuture};
use super::{attr::AttributeValue, VNode, traits::RenderToStream};

#[derive(Default)]
pub struct ElementAttributes(std::collections::HashMap<String, AttributeValue>);

impl<'a> FromIterator<(&'a str, AttributeValue)> for ElementAttributes {
    fn from_iter<T: IntoIterator<Item = (&'a str, AttributeValue)>>(iter: T) -> Self {
        Self(iter.into_iter().map(|(name, val)| (name.to_owned(), val)).collect())
    }
}

impl FromIterator<(String, AttributeValue)> for ElementAttributes {
    fn from_iter<T: IntoIterator<Item = (String, AttributeValue)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> RenderToStream<'a> for &'a ElementAttributes {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async {
            
            for (attr_name, attr_value) in self.0.iter().filter(|(_, attr_value)| attr_value.is_literal()) {
                stream.write(" ".as_bytes()).await?;
                stream.write_all(attr_name.as_bytes()).await?;
                stream.write_all("=".as_bytes()).await?;
                attr_value.render_to_stream(stream).await?;
            }

            Ok(())
        })
    }
}

pub struct ElementNode {
    tag: String,
    attributes: ElementAttributes,
    children: Vec<VNode>
}

impl Into<VNode> for ElementNode {
    fn into(self) -> VNode {
        VNode::Element(self)
    }
}

impl ElementNode {
    pub fn new<'a, Str, Attrs, Children>(tag: Str, attributes: Attrs, children: Children) -> Self 
    where Str: Into<String>, Attrs: IntoIterator<Item=(&'a str, AttributeValue)>, Children: IntoIterator<Item=VNode>
    {
        Self {
            tag: tag.into(),
            attributes: attributes.into_iter().collect(),
            children: children.into_iter().collect()
        }
    }
}

impl<'a> RenderToStream<'a> for &'a ElementNode {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> futures::prelude::future::LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async {
            stream.write_all("<".as_bytes()).await?;
            stream.write_all(self.tag.as_bytes()).await?;
            self.attributes.render_to_stream(stream).await?;
            stream.write_all(">".as_bytes()).await?;

            for child in self.children.iter() {
                child
                .render_to_stream(stream)
                .await?;
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
    use crate::vdom::traits::RenderToStream;

    use super::ElementNode;
    use futures::io::AllowStdIo;
    use std::io::{BufWriter, Cursor};

    #[tokio::test]
    async fn test_render_to_stream() {
        let mut output = Vec::<u8>::default();

        {
            let mut stream = AllowStdIo::new(
                BufWriter::new(
                    Cursor::new(&mut output)
                )
            );
            
            let el = ElementNode::new(
                "div", 
                [("class", "px-2".into())], 
                [ElementNode::new(
                    "p",
                    [],
                    []
                ).into()]
            );

            el.render_to_stream(&mut stream).await.unwrap();
        }


        let str = String::from_utf8(output).unwrap();
        assert_eq!(str, "<div class=\"px-2\"><p></p></div>");
    }
}