#[cfg(test)]
mod tests {

    use json_api::{Resource, IdentifierObject};
    use json_api_derive::Resource;

    #[derive(Debug, Resource)]
    struct Pancakes {
        name: String,
        sauce: usize
    }

    #[test]
    fn it_works() {
        let string = "pancakes".to_string();
        let pancake = Pancakes {
            name: "pancakes".to_string(),
            sauce: 0
        };
        println!("{:?}", pancake.to_identifier());
        println!("{:?}", IdentifierObject::from(pancake));

//        assert_eq!(pancake.to_identifier(), string);
    }
}
