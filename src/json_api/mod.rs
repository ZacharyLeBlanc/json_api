use super::data::kind::DataType;

pub struct JsonApi {
    data: Option<DataType>,
    errors: Option<String>,
    meta: Option<String>,
    jsonapi: Option<String>,
    links: Option<String>,
    included: Option<String>
}

impl JsonApi {
    pub fn new() -> Self {
        JsonApi {
            data: None,
            errors: None,
            meta: None,
            jsonapi: None,
            links: None,
            included: None
        }
    }
}