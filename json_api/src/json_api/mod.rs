mod data;
use crate::resource::ResourceType;
use crate::ResourceTrait;
use data::Data;
pub use data::DataType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonApi {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Data>,
    #[serde(skip_serializing)]
    data_type: DataType,
}

impl JsonApi {
    pub fn new() -> Self {
        JsonApi {
            data: None,
            data_type: DataType::default(),
        }
    }

    pub fn add_data(&mut self, resource: impl ResourceTrait) {
        match &self.data {
            None => match self.data_type {
                DataType::Identifier => {
                    self.data = Some(Data::Singular(ResourceType::Identifier(
                        resource.to_identifier(),
                    )))
                }
                DataType::Object => {
                    self.data = Some(Data::Singular(ResourceType::Object(
                        resource.to_resource_object(),
                    )))
                }
            },
            Some(data) => match data {
                Data::Singular(resource_type) => match resource_type {
                    ResourceType::Object(object) => {
                        let array = vec![
                            ResourceType::Object(object.clone()),
                            ResourceType::Object(resource.to_resource_object()),
                        ];
                        self.data = Some(Data::Array(array));
                    }

                    ResourceType::Identifier(identifier) => {
                        let array = vec![
                            ResourceType::Identifier(identifier.clone()),
                            ResourceType::Identifier(resource.to_identifier()),
                        ];
                        self.data = Some(Data::Array(array));
                    }
                },
                Data::Array(resource_types) => match self.data_type {
                    DataType::Identifier => {
                        let mut data = resource_types.clone();
                        data.push(ResourceType::Identifier(resource.to_identifier()));
                        self.data = Some(Data::Array(data));
                    }

                    DataType::Object => {
                        let mut data = resource_types.clone();
                        data.push(ResourceType::Object(resource.to_resource_object()));
                        self.data = Some(Data::Array(data));
                    }
                },
            },
        }
    }

    pub fn add_collection(&self) {}
}
