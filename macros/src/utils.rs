use proc_macro2::TokenStream;
use syn::FieldsNamed;
use syn::parse::Parser;

pub(crate) fn add_field(fields: &mut FieldsNamed, tokens: TokenStream) {
  fields.named.push(syn::Field::parse_named.parse2(tokens).unwrap());
}
