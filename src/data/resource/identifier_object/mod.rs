use super::ResourceIdentifier;

#[derive(Clone, Debug, PartialEq)]
struct ResourceIdentifierObject {
    r#type: &'static str,
    id: String
}

impl ResourceIdentifierObject {
    pub fn new(object_type: &'static str, id: String) -> Self {
        ResourceIdentifierObject {
            r#type: object_type,
            id
        }
    }

    pub fn from(resource: impl ResourceIdentifier) -> Self {
        ResourceIdentifierObject {
            r#type: resource.get_type(),
            id: resource.get_id()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ResourceIdentifierObject;
    use super::super::ResourceIdentifier;

    #[derive(Debug, Hello)]
    struct Article {
        id: String
    }

    impl ResourceIdentifier for Article {
        fn get_id(&self) -> String {
            self.id.clone()
        }

        fn get_type(&self) -> &'static str {
            "article"
        }
    }

    #[test]
    fn create_new_resource_identifier_object_from_new() {
        let article = ResourceIdentifierObject::new("article", "1".to_string());
        let article_resource = ResourceIdentifierObject {
            r#type: "article",
            id: "1".to_string()
        };
        assert_eq!(article, article_resource)
    }

    #[test]
    fn create_new_resource_identifier_object_from_resource() {
        let resource = Article {
            id: "1".to_string()
        };
        let article = ResourceIdentifierObject::from(resource);
        let article_resource = ResourceIdentifierObject {
            r#type: "article",
            id: "1".to_string()
        };
        assert_eq!(article, article_resource)
    }
}