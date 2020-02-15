use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Link<'a, 'b> {
    String(&'a str, &'b str),
    Object(&'a str, &'b str /*, meta*/),
    SelfReference(&'a str),
    SelfReferenceObject(&'a str /*, meta*/),
    About(&'a str),
    AboutObject(&'a str /*, meta*/),
    First(&'a str),
    Last(&'a str),
    Previous(&'a str),
    Next(&'a str),
    Related(&'a str),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkObject {
    href: String,
    // TODO: Meta
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum LinkType {
    String(String),
    Object(LinkObject),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    #[serde(flatten)]
    links: HashMap<String, LinkType>,
}

impl Links {
    pub fn new(links: Vec<Link>) -> Self {
        let mut link_map = HashMap::new();
        for link in links {
            match link {
                Link::String(key, href) => {
                    link_map.insert(key.to_string(), LinkType::String(href.to_string()));
                }
                Link::Object(key, href) => {
                    link_map.insert(
                        key.to_string(),
                        LinkType::Object(LinkObject {
                            href: href.to_string(),
                        }),
                    );
                }
                Link::SelfReference(href) => {
                    link_map.insert("self".to_string(), LinkType::String(href.to_string()));
                }
                Link::SelfReferenceObject(href) => {
                    link_map.insert(
                        "self".to_string(),
                        LinkType::Object(LinkObject {
                            href: href.to_string(),
                        }),
                    );
                }
                Link::About(href) => {
                    link_map.insert("about".to_string(), LinkType::String(href.to_string()));
                }
                Link::AboutObject(href) => {
                    link_map.insert(
                        "about".to_string(),
                        LinkType::Object(LinkObject {
                            href: href.to_string(),
                        }),
                    );
                }

                Link::First(href) => {
                    link_map.insert("first".to_string(), LinkType::String(href.to_string()));
                }
                Link::Last(href) => {
                    link_map.insert("last".to_string(), LinkType::String(href.to_string()));
                }
                Link::Previous(href) => {
                    link_map.insert("prev".to_string(), LinkType::String(href.to_string()));
                }
                Link::Next(href) => {
                    link_map.insert("next".to_string(), LinkType::String(href.to_string()));
                }
                Link::Related(href) => {
                    link_map.insert("related".to_string(), LinkType::String(href.to_string()));
                }
            }
        }
        Links { links: link_map }
    }
}
