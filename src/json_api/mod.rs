mod builder;
use builder::JsonApiBuilder;

#[derive(Debug, PartialEq)]
pub struct JsonApi {
//    data: Option<DataType>,
    errors: Option<bool>,
    meta: Option<bool>,
    jsonapi: Option<bool>,
    links: Option<bool>,
    included: Option<bool>
}

pub fn get_instance() -> JsonApiBuilder {
    JsonApiBuilder::new()
}
