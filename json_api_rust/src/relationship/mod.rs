use crate::json_api::Data;
use crate::Links;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Relationship {
    links: Links,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Data>,
}

impl Relationship {
    pub fn new(links: Links, data: Option<Data>) -> Self {
        Relationship { links, data }
    }
}
