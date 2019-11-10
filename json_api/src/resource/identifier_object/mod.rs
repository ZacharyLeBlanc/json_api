use super::Resource;

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierObject {
    r#type: String,
    id: String
}

impl IdentifierObject {
    pub fn new(r#type: String, id: String) -> Self {
        IdentifierObject {
            r#type,
            id
        }
    }

    pub fn from(resource: impl Resource) -> IdentifierObject {
        resource.to_identifier()
    }
}