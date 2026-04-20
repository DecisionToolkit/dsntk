#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate quote;

mod business_context_element;
mod dmn_element;
mod expression;
mod named_element;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_derive(ToErrorMessage)]
pub fn to_error_message(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl ToErrorMessage for #name {
      fn message(self) -> String {
        self.0
      }
    }
  };
  TokenStream::from(expanded)
}

/// Extends annotated struct with fields and methods required by DMN named element.
#[proc_macro_attribute]
pub fn named_element(_input: TokenStream, item: TokenStream) -> TokenStream {
  named_element::process(item.into()).into()
}

/// Extends annotated struct with fields and methods required by DMN element.
#[proc_macro_attribute]
pub fn dmn_element(_: TokenStream, item: TokenStream) -> TokenStream {
  dmn_element::process(item.into()).into()
}

/// Extends annotated struct with fields and methods required by DMN business context element.
#[proc_macro_attribute]
pub fn business_context_element(_: TokenStream, item: TokenStream) -> TokenStream {
  business_context_element::process(item.into()).into()
}

/// Extends annotated struct with fields and methods required by DMN expressions.
#[proc_macro_attribute]
pub fn expression(_input: TokenStream, item: TokenStream) -> TokenStream {
  expression::process(item.into()).into()
}
