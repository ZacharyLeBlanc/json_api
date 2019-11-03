use super::resource::object::ResourceObject;
use super::resource::identifier_object::ResourceIdentifierObject;

pub enum DataType {
    ResourceObject(ResourceObject),
    ResourceIdentifierObject(ResourceIdentifierObject),
    ResourceObjects(Vec<ResourceObject>),
    ResourceIdentifierObjects(Vec<ResourceIdentifierObject>)
}