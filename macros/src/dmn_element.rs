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
        /// The namespace the DMN element belongs to.
        pub(crate) namespace: Uri
      },
    );
    add_field(
      &mut fields,
      quote! {
        /// Name of the model the DMN element was defined in.
        pub(crate) model_name: String
      },
    );
    add_field(
      &mut fields,
      quote! {
        /// Optional identifier of the DMN element.
        pub(crate) id: DmnId
      },
    );
    add_field(
      &mut fields,
      quote! {
        /// Optional description of the DMN element.
        pub(crate) description: Option<String>
      },
    );
    add_field(
      &mut fields,
      quote! {
        /// Optional alternative short description of the DMN element.
        pub(crate) label: Option<String>
      },
    );
    add_field(
      &mut fields,
      quote! {
        /// Container to attach additional elements to DMN element.
        pub(crate) extension_elements: Vec<ExtensionElement>
      },
    );
    add_field(
      &mut fields,
      quote! {
        /// Container to attach named extended attributes and model associations to DMN element.
        pub(crate) extension_attributes: Vec<ExtensionAttribute>
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
      impl DmnElement for #name {
        fn namespace(&self) -> &str {
          &self.namespace
        }
        fn model_name(&self) -> &str {
          &self.model_name
        }
        fn id(&self) -> &String {
          match &self.id {
            DmnId::Provided(id) => id,
            DmnId::Generated(id) => id,
          }
        }
        fn opt_id(&self) -> Option<&String> {
          match &self.id {
            DmnId::Provided(id) => Some(&id),
            DmnId::Generated(_) => None,
          }
        }
        fn description(&self) -> &Option<String> {
          &self.description
        }
        fn label(&self) -> &Option<String> {
          &self.label
        }
        fn extension_elements(&self) -> &Vec<ExtensionElement> {
          &self.extension_elements
        }
        fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
          &self.extension_attributes
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
