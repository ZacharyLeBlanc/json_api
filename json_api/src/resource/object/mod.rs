use crate::link::Links;
use crate::relationship::Relationship;
use crate::ResourceTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceObject {
    r#type: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationships: Option<HashMap<String, Relationship>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<String>,
}

impl ResourceObject {
    pub fn new(r#type: String, id: String) -> Self {
        ResourceObject {
            r#type,
            id,
            attributes: None,
            relationships: None,
            links: None,
            meta: None,
        }
    }

    pub fn relationships(&mut self, relationships: Option<HashMap<String, Relationship>>) {
        self.relationships = relationships;
    }

    pub fn from(resource: impl ResourceTrait) -> ResourceObject {
        resource.to_resource_object()
    }

    pub fn add_attributes(&mut self, attributes: Option<HashMap<String, Value>>) {
        self.attributes = attributes;
    }
}
