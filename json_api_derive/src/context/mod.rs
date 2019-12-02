mod symbol;
use symbol::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::export::ToTokens;
use syn::{
    Attribute,
    Data::Struct,
    DeriveInput, Error, Field, Fields, Ident,
    Meta::{List, Path},
    NestedMeta::Meta,
    Type,
};

pub struct ResourceContext {
    errors: Vec<Error>,
    fields: Vec<(Ident, Type)>,
    id_field: Option<Ident>,
    resource_name: Option<Ident>,
}

impl ResourceContext {
    pub fn new() -> Self {
        ResourceContext {
            errors: vec![],
            fields: vec![],
            id_field: None,
            resource_name: None,
        }
    }

    pub fn from(ast: &DeriveInput) -> ResourceContext {
        let mut context = ResourceContext::new();
        parse(&ast, &mut context);
        context
    }

    pub fn add_error(&mut self, err: Error) {
        self.errors.push(err);
    }

    pub fn check(&self) -> Result<(), Vec<Error>> {
        let errors = self.errors.clone();
        match errors.len() {
            0 => Ok(()),
            _ => Err(errors),
        }
    }

    pub fn to_token_stream(&self) -> TokenStream {
        match self.check() {
            Ok(_) => {
                let name = self.resource_name.as_ref().unwrap();
                let id_field = self.id_field.as_ref().unwrap();
                let fields = &self.fields;
                let mut definitions = vec![];
                for (ident, _ty) in fields {
                    let name = ident.to_string();
                    definitions.push(quote!(map.insert(#name.to_string(), serde_json::json!(self.#ident.clone()));));
                }

                let mut concatenated = format!("{}_ResourceIdentifierObject", name);
                let resource_identifier_object = Ident::new(&concatenated, name.span());
                concatenated = format!("{}_ResourceObject", name);
                let resource_object = Ident::new(&concatenated, name.span());
                concatenated = format!("{}_ResourceTrait", name);
                let resource_trait = Ident::new(&concatenated, name.span());

                return quote! {
                    use json_api::{ResourceIdentifierObject as #resource_identifier_object, ResourceObject as #resource_object, ResourceTrait as #resource_trait};

                    impl #resource_trait for #name {
                        fn get_attributes(&self) -> Option<std::collections::HashMap<String, serde_json::Value>> {
                            let mut map = std::collections::HashMap::new();
                            #(#definitions)*
                            Some(map)
                        }

                        fn get_id(&self) -> String {
                            self.#id_field.to_string()
                        }

                        fn to_identifier(&self) -> #resource_identifier_object {
                            <#resource_identifier_object>::new(stringify!(#name).to_lowercase(), self.get_id())
                        }

                        fn to_resource_object(&self) -> #resource_object {
                            let mut object = <#resource_object>::new(stringify!(#name).to_lowercase(), self.get_id());
                            object.add_attributes(self.get_attributes());
                            object
                        }
                    }
                };
            }
            Err(errors) => to_compile_errors(errors),
        }
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}

fn parse(ast: &DeriveInput, context: &mut ResourceContext) {
    parse_name(ast, context);
    parse_data(ast, context);
}

fn parse_data(ast: &DeriveInput, context: &mut ResourceContext) {
    match &ast.data {
        Struct(data) => {
            parse_fields(&data.fields, context);
            if context.id_field.is_none() {
                context.add_error(Error::new_spanned(
                    ast.to_token_stream(),
                    "an id field is required for this resource",
                ));
            }
        }
        _ => context.add_error(Error::new(ast.ident.span(), "Expected Struct")),
    }
}

fn parse_field_attributes(field: &Field, attributes: &[Attribute], context: &mut ResourceContext) {
    for attribute in attributes {
        if let Ok(metadata) = attribute.parse_meta() {
            if let List(meta_list) = metadata {
                for nested_metadata in meta_list.nested.iter() {
                    match nested_metadata {
                        Meta(Path(word)) if word.clone() == ID => {
                            if let Some(ident) = &field.ident {
                                match context.id_field {
                                    None => {
                                        context.id_field = Some(Ident::new(
                                            ident.to_string().as_str(),
                                            ident.span(),
                                        ));
                                    }
                                    Some(_) => context.add_error(Error::new_spanned(
                                        field.to_token_stream(),
                                        "only one id field is expected",
                                    )),
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn parse_fields(fields: &Fields, context: &mut ResourceContext) {
    if let Fields::Named(named_fields) = fields {
        for field in named_fields.named.iter() {
            parse_field_attributes(field, &field.attrs, context);
            if let Some(ident) = &field.ident {
                if ident != context.id_field.as_ref().unwrap() {
                    context.fields.push((ident.clone(), field.ty.clone()));
                }
            };
        }
    };
}

fn parse_name(ast: &DeriveInput, context: &mut ResourceContext) {
    context.resource_name = Some(Ident::new(
        &ast.ident.to_string().as_str(),
        ast.ident.span(),
    ));
}
