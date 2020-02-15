mod symbol;
use symbol::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::export::ToTokens;
use syn::{
    Attribute,
    Data::Struct,
    DeriveInput, Error, Field, Fields, Ident,
    Lit::Str,
    Meta::{List, NameValue, Path},
    NestedMeta::Meta,
};

pub struct ResourceContext {
    url: Option<String>,
    errors: Vec<Error>,
    fields: Vec<Ident>,
    id_field: Option<Ident>,
    relationship_fields: Vec<Ident>,
    resource_name: Option<Ident>,
}

impl ResourceContext {
    pub fn new() -> Self {
        ResourceContext {
            url: None,
            errors: vec![],
            fields: vec![],
            id_field: None,
            relationship_fields: vec![],
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
                for ident in fields {
                    let name = ident.to_string();
                    definitions.push(quote!(map.insert(#name.to_string(), serde_json::json!(self.#ident.clone()));));
                }

                let relationships: Vec<TokenStream> = self.relationship_fields.iter().map(|ident| {
                    let resource = name.to_string().to_lowercase();
                    let name = ident.to_string();
                    let x = quote!(Some(json_api::Data::Singular(json_api::ResourceType::Identifier(self.#ident.clone().to_identifier()))));
                    quote!(relationships.insert(#name.to_string(), json_api::Relationship::new(json_api::Links::new(vec![json_api::Link::Related(&(format!("/{}/{}/{}", #resource, self.get_id(), #name)))]), #x));)
               }).collect();

                let add_relationships = if self.relationship_fields.is_empty() {
                    quote!(object.relationships(None);)
                } else {
                    quote! {
                     let mut relationships = std::collections::HashMap::new();
                      #(#relationships)*
                      object.relationships(Some(relationships));
                    }
                };

                return quote! {
                    use json_api::*;

                    impl ResourceTrait for #name {
                        fn get_attributes(&self) -> Option<std::collections::HashMap<String, serde_json::Value>> {
                            let mut map = std::collections::HashMap::new();
                            #(#definitions)*
                            Some(map)
                        }

                        fn get_id(&self) -> String {
                            self.#id_field.to_string()
                        }

                        fn to_identifier(&self) -> ResourceIdentifierObject {
                            ResourceIdentifierObject::new(stringify!(#name).to_lowercase(), self.get_id())
                        }

                        fn to_resource_object(&self) -> ResourceObject {
                            let mut object = ResourceObject::new(stringify!(#name).to_lowercase(), self.get_id());
                            object.add_attributes(self.get_attributes());
                            #add_relationships
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
    parse_attrs(ast, context);
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
                        Meta(Path(word)) if word.clone() == TO_ONE => {
                            if let Some(ident) = &field.ident {
                                context
                                    .relationship_fields
                                    .push(Ident::new(ident.to_string().as_str(), ident.span()))
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
                if ident != context.id_field.as_ref().unwrap()
                    && !context.relationship_fields.contains(ident)
                {
                    context.fields.push(ident.clone());
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

fn parse_attrs(ast: &DeriveInput, context: &mut ResourceContext) {
    ast.attrs.iter().for_each(|attr| {
        if let Ok(metadata) = attr.parse_meta() {
            if let List(meta_list) = metadata {
                for nested_metadata in meta_list.nested.iter() {
                    if let Meta(meta) = nested_metadata {
                        if let NameValue(name_value) = meta {
                            if name_value.path == URL {
                                if let Str(string) = &name_value.lit {
                                    context.url = Some(string.value());
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
