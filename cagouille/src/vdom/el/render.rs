use futures::{future::LocalBoxFuture, AsyncWrite, AsyncWriteExt};

use crate::vdom::traits::RenderToStream;

use super::ElementNode;


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