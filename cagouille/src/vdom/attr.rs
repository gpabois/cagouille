use futures::{AsyncWriteExt, AsyncWrite, io::BufWriter, future::LocalBoxFuture};
use wasm_bindgen::JsValue;

use super::traits::RenderToStream;

pub enum AttributeValue {
    JsValue(JsValue),
    String(String),
    Boolean(bool)
}

impl From<JsValue> for AttributeValue {
    fn from(value: JsValue) -> Self {
        Self::JsValue(value)
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl AttributeValue {
    pub fn is_literal(&self) -> bool {
        match self {
            Self::JsValue(js_val) => {
                js_val.as_f64().is_some() ||
                js_val.as_bool().is_some() ||
                js_val.as_string().is_some()
            },
            Self::String(_) => true,
            Self::Boolean(_) => true
        }
    }
}

impl<'a> RenderToStream<'a> for &'a AttributeValue {
    fn render_to_stream<'stream, 'fut, W: AsyncWrite + AsyncWriteExt + Unpin>(self, stream: &'stream mut W) 
    -> LocalBoxFuture<'fut, Result<(), std::io::Error>>
    where 'a: 'fut, 'stream: 'fut {
        Box::pin(async move {
            match self {
                AttributeValue::JsValue(js_val) => {
                    if let Some(mut str) = js_val.as_string() {
                        str = str.replace("\"", "\\\"");
                        stream.write_all(str.as_bytes()).await?;
                    }
                    if let Some(b) = js_val.as_bool() {
                        stream.write_all(format!("\"{b}\"").as_bytes()).await?;
                    }
                    if let Some(f) = js_val.as_f64() {
                        stream.write_all(format!("\"{f}\"").as_bytes()).await?;
                    }
                },
                AttributeValue::Boolean(b) => {
                    stream.write_all(format!("\"{b}\"").as_bytes()).await?;
                },
                AttributeValue::String(ref_str) => {
                    let str = ref_str.replace("\"", "\\\"");
                    stream.write_all(format!("\"{}\"", str).as_bytes()).await?;
                }
            }

            Ok(())
        })
    }
}

