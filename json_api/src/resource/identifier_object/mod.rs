use super::ResourceTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceIdentifierObject {
    r#type: String,
    id: String,
}

impl ResourceIdentifierObject {
    pub fn new(r#type: String, id: String) -> Self {
        ResourceIdentifierObject { r#type, id }
    }

    pub fn from(resource: impl ResourceTrait) -> ResourceIdentifierObject {
        resource.to_identifier()
    }
}
