//use super::JsonApi;

pub struct JsonApiBuilder {
}

impl JsonApiBuilder {
    pub fn new() -> Self {
        JsonApiBuilder {
        }
    }

//    pub fn build(&self) -> Result<JsonApi, String> {
//        if !self.is_valid() {
//            Err("A document MUST contain at least one of the following top-level members: data, errors, or meta.".to_string())
//        } else {
//
////            Ok(JsonApi {
////                data: self.data,
////                errors: self.errors,
////                meta: self.meta,
////                jsonapi: self.jsonapi,
////                links: self.links,
////                included: self.included
////            })
//        }
//    }

//    fn is_valid(&self) -> bool {
//        let contains_top_level_members = self.data.is_some() || self.errors.is_some() || self.meta.is_some();
//        let data_error_coexist = self.data.is_some() && self.errors.is_some();
//        contains_top_level_members && !data_error_coexist
//    }
}


