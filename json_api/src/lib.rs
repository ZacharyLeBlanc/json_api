mod error;
pub use error::{JsonApiError, Source};
mod json_api;
pub use crate::json_api::{data::Data, data::DataType, JsonApi};
mod link;
pub use link::{Link, Links};
pub mod meta;
pub mod resource;
pub use resource::{ResourceIdentifierObject, ResourceObject, ResourceTrait, ResourceType};
pub mod relationship;
pub use relationship::Relationship;

#[cfg(feature = "json_api_derive")]
pub use json_api_derive::Resource;
