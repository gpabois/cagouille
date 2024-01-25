use syn::{Data, Field};

pub fn iter_fields<'a>(data: &'a Data) -> Box<dyn Iterator<Item=&'a Field> + 'a> {
    if let syn::Data::Struct(s) = data {
        match &s.fields {
            syn::Fields::Named(fields) => return Box::new(fields.named.iter()),
            syn::Fields::Unnamed(fields) => return Box::new(fields.unnamed.iter()),
            _ => {}
        }
    }

    return Box::new(std::iter::empty())
}