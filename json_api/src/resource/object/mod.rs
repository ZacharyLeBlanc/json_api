use crate::ResourceTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceObject {
    r#type: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationships: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<String>,
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

    pub fn from(resource: impl ResourceTrait) -> ResourceObject {
        resource.to_resource_object()
    }

    pub fn add_attributes(&mut self, attributes: Option<HashMap<String, Value>>) {
        self.attributes = attributes;
    }
}
