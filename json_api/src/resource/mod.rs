pub mod identifier_object;
pub mod object;
pub use identifier_object::ResourceIdentifierObject;
pub use object::ResourceObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceType {
    Identifier(ResourceIdentifierObject),
    Object(ResourceObject),
}

pub trait ResourceTrait {
    fn get_attributes(&self) -> Option<HashMap<String, Value>> {
        None
    }
    fn get_id(&self) -> String;
    fn to_identifier(&self) -> ResourceIdentifierObject;
    fn to_resource_object(&self) -> ResourceObject;
}
