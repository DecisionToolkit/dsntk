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
        /// Name of the named element.
        pub(crate) name: String
      },
    );
    add_field(
      &mut fields,
      quote! {
        /// FEEL name of the named element.
        pub(crate) feel_name: Name
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
      impl NamedElement for #name {
        /// Returns a name of this named element.
        fn name(&self) -> &str {
          &self.name
        }
        /// Returns a FEEL name of this named element.
        fn feel_name(&self) -> &Name {
          &self.feel_name
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
