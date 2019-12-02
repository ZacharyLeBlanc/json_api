extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod context;
use context::ResourceContext;

#[proc_macro_derive(Resource, attributes(json_api))]
pub fn resource_macro_derive(input: TokenStream) -> TokenStream {
    impl_resource_macro(parse_macro_input!(input as DeriveInput))
}

fn impl_resource_macro(ast: DeriveInput) -> TokenStream {
    ResourceContext::from(&ast).to_token_stream().into()
}
