pub(crate) mod data;
use crate::error::JsonApiError;
use crate::link::Links;
use crate::resource::ResourceType;
use crate::{Link, ResourceTrait};
pub use data::Data;
pub use data::DataType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonApi {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<JsonApiError>>,
    // TODO: Meta
    #[serde(borrow)]
    jsonapi: JsonApiObject<'static>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included: Option<Data>,
}

impl JsonApi {
    pub fn new() -> Self {
        JsonApi {
            ..Default::default()
        }
    }

    pub fn data(resource: impl ResourceTrait) -> Self {
        JsonApi {
            data: Some(Data::Singular(ResourceType::Object(
                resource.to_resource_object(),
            ))),
            ..Default::default()
        }
    }

    pub fn collection(resources: Vec<impl ResourceTrait>) -> Self {
        JsonApi {
            data: Some(Data::Array(
                resources
                    .iter()
                    .map(|resource| ResourceType::Object(resource.to_resource_object()))
                    .collect(),
            )),
            ..Default::default()
        }
    }

    pub fn error(error: JsonApiError) -> Self {
        JsonApi {
            errors: Some(vec![error]),
            ..Default::default()
        }
    }

    pub fn errors(errors: Vec<JsonApiError>) -> Self {
        JsonApi {
            errors: Some(errors),
            ..Default::default()
        }
    }

    pub fn links(&mut self, links: Vec<Link>) -> &mut Self {
        self.links = Some(Links::new(links));
        self
    }

    pub fn to_response(&self) -> Result<Self, Vec<ValidationError>> {
        self.clone().validate()
    }

    fn validate(self) -> Result<Self, Vec<ValidationError>> {
        let mut errors = vec![];
        if self.data.is_none() && self.errors.is_none() {
            errors.push(ValidationError::MissingTopLevelMembers);
        }
        if self.data.is_some() && self.errors.is_some() {
            errors.push(ValidationError::DataErrorCoexist);
        }
        if self.data.is_none() && self.included.is_some() {
            errors.push(ValidationError::MissingDataAndIncluded);
        }
        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug)]
pub enum ValidationError {
    MissingTopLevelMembers,
    DataErrorCoexist,
    MissingDataAndIncluded,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JsonApiObject<'a> {
    version: &'a str,
    // TODO: Meta
}

impl Default for JsonApiObject<'_> {
    fn default() -> Self {
        JSON_API_OBJECT
    }
}

const JSON_API_OBJECT: JsonApiObject<'static> = { JsonApiObject { version: "1.0" } };
