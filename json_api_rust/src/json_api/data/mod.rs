use crate::resource::ResourceType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Singular(ResourceType),
    Array(Vec<ResourceType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Identifier,
    Object,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::Object
    }
}
