#[cfg(test)]
mod tests {

    use json_api::{JsonApi, Resource};
    #[test]
    fn it_works() {
        let mut json_api = JsonApi::new();
        let article = Articles {
            id: 0,
            title: "lol".to_string(),
            comment: "hi".to_string(),
            number: 0,
            array: vec![],
        };
        let article2 = Articles {
            id: 2,
            title: "hihi".to_string(),
            comment: "byue".to_string(),
            number: 0,
            array: vec![],
        };
        let article3 = Articles {
            id: 3,
            title: "hihi".to_string(),
            comment: "byue".to_string(),
            number: 0,
            array: vec![],
        };
        json_api.add_data(article);
        let mut serialized = serde_json::to_string(&json_api).unwrap();
        println!("{}", serialized);
        json_api.add_data(article2);
        serialized = serde_json::to_string(&json_api).unwrap();
        println!("{}", serialized);
        json_api.add_data(article3);
        serialized = serde_json::to_string(&json_api).unwrap();
        println!("{}", serialized);
    }

    #[derive(Debug, Resource)]
    struct Articles {
        #[json_api(id)]
        id: usize,
        title: String,
        comment: String,
        number: usize,
        array: Vec<usize>,
    }

    #[test]
    fn it_implements_to_identifier() {
        //        let article = Articles {
        //            id: 0,
        //            title: "Rails is Omakase".to_string(),
        //            comment: "hi".to_string(),
        //            number: 9,
        //            array: vec![],
        //        };
        //        let article_identifier =
        //            ResourceIdentifierObject::new("articles".to_string(), "0".to_string());
        //        assert_eq!(article.to_identifier(), article_identifier);
        //        assert_eq!(ResourceIdentifierObject::from(article), article_identifier);
    }

    #[test]
    fn it_implements_get_attributes() {
        let article = Articles {
            id: 0,
            title: "Rails is Omakase".to_string(),
            comment: "hi".to_string(),
            number: 9,
            array: vec![0, 1, 2, 3],
        };
        println!("{:#?}", article.to_resource_object());
    }
}
