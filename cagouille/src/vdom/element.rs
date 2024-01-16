use futures::{AsyncWriteExt, AsyncWrite, io::BufWriter, future::LocalBoxFuture};

use super::{attr::AttributeValue, VNode, traits::RenderToStream};

#[derive(Default)]
pub struct ElementAttributes(std::collections::HashMap<String, AttributeValue>);

impl<'a> RenderToStream<'a> for &'a ElementAttributes {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + Unpin>(self, stream: &'stream mut BufWriter<W>) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        todo!()
    }
}

pub struct ElementNode {
    tag: String,
    attributes: ElementAttributes,
    children: Vec<VNode>
}

impl<'a> RenderToStream<'a> for &'a ElementNode {
    fn render_to_stream<'stream, 'fut, W: futures::prelude::AsyncWrite + Unpin>(self, stream: &'stream mut futures::io::BufWriter<W>) 
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
    
}