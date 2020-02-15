use crate::link::{Link, Links};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
    // TODO: Meta
}

impl JsonApiError {
    pub fn new() -> Self {
        JsonApiError {
            id: None,
            links: None,
            status: None,
            code: None,
            title: None,
            detail: None,
            source: None,
        }
    }

    pub fn finish(&self) -> Self {
        self.clone()
    }

    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn links(&mut self, links: Vec<Link>) -> &mut Self {
        let mut error = "";
        let (mut has_about, mut has_pagination) = (false, false);
        links.iter().for_each(|link| match link {
            Link::About(_) | Link::AboutObject(_) => {
                error = "When returning an error you need to have an about link.";
                has_about = true
            }
            Link::First(_) | Link::Last(_) | Link::Previous(_) | Link::Next(_) => {
                error = "Errors should not have pagination links.";
                has_pagination = true
            }
            _ => (),
        });
        if !has_about || has_pagination {
            panic!(error)
        }
        self.links = Some(Links::new(links));
        self
    }

    pub fn status(&mut self, status: u32) -> &mut Self {
        self.status = Some(status.to_string());
        self
    }

    pub fn code(&mut self, code: isize) -> &mut Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn detail(&mut self, detail: &str) -> &mut Self {
        self.detail = Some(detail.to_string());
        self
    }

    pub fn source(&mut self, source: Source) -> &mut Self {
        self.source = Some(source);
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Source {
    Pointer(String),
    Parameter(String),
}
