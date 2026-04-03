use crate::utils::add_field;
use proc_macro2::TokenStream;
use syn::fold::Fold;
use syn::{FieldsNamed, ItemStruct, parse2};

struct Processor;

impl Fold for Processor {
  fn fold_fields_named(&mut self, mut fields: FieldsNamed) -> FieldsNamed {
    add_field(
      &mut fields,
      quote! {
        /// Optional type definition.
        pub (crate) type_ref: Option<String>
      },
    );
    fields
  }
}

pub(crate) fn process(item: TokenStream) -> TokenStream {
  fn process_item(item: TokenStream) -> syn::Result<TokenStream> {
    let input: ItemStruct = parse2(item)?;
    let input = Processor.fold_item_struct(input);
    let name = &input.ident;
    let expanded = quote! {
      impl Expression for #name {
        /// Returns a reference to optional type definition.
        fn type_ref(&self) -> &Option<String> {
          &self.type_ref
        }
      }
    };
    Ok(quote! {
      #input
      #expanded
    })
  }

  process_item(item).unwrap_or_else(syn::Error::into_compile_error)
}
