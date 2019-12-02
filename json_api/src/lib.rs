mod json_api;
pub mod resource;
pub use json_api::JsonApi;
pub use resource::{ResourceIdentifierObject, ResourceObject, ResourceTrait};

pub use json_api_derive::Resource;
