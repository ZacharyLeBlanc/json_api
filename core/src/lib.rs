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

#[cfg(feature = "derive")]
pub use j_api_derive::Resource;
#[cfg(feature = "actix")]
pub use j_api_actix::JsonApiResponse;