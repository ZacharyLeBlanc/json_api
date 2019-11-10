pub mod identifier_object;
pub use identifier_object::{ IdentifierObject };

pub trait Resource {
    fn get_id(&self) -> String;
    fn to_identifier(&self) -> IdentifierObject;
}