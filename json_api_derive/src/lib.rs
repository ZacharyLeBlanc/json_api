extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{Data, DeriveInput, Error, Fields, Ident, parse_macro_input};

use quote::quote;

#[proc_macro_derive(Resource)]
pub fn resource_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the trait implementation
    impl_resource_macro(input)
}

fn get_name(ast: &DeriveInput) -> &Ident {
    &ast.ident
}

fn parse_data(ast: &DeriveInput) -> Result<Vec<String>, Error> {
    match &ast.data {
        Data::Struct(data_struct) => {
            Ok(parse_fields(&data_struct.fields))
        }
        _ => {
            Err(Error::new(ast.ident.span(), "a resource may only be a struct"))
        }
    }
}

fn parse_fields(fields: &Fields) -> Vec<String> {
    match fields {
        syn::Fields::Named(fields_named) => {
            let mut field_names = vec![];
            let named = &fields_named.named;
            for field in named.iter() {
                field.ident.as_ref().map(|ident| {
                    field_names.push(ident.to_string())
                });
            }
            field_names
        }
        _ => {
            vec![]
        }
    }
}

fn impl_resource_macro(ast: DeriveInput) -> TokenStream {
    let name = get_name(&ast);
    match parse_data(&ast) {
        Ok(field_names) => {
            let gen = quote! {
                impl Resource for #name {
                    fn get_id(&self) -> String {
                        "hi".to_string()
                    }

                    fn to_identifier(&self) -> IdentifierObject {
                        IdentifierObject::new(stringify!(#name).to_lowercase(), self.get_id())
                    }
                }
            };
            TokenStream::from(gen)
        }
        Err(error) => {
            error.to_compile_error().into()
        }
    }
}