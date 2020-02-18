use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use j_api::{JsonApi, JsonApiError, Link};

#[derive(Debug, Clone)]
pub enum JsonApiResponse {
    Ok(JsonApi),
    Err(StatusCode, Vec<JsonApiError>),
}

// Responder
impl Responder for JsonApiResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let (status, mut response) = match self {
            JsonApiResponse::Ok(json_api) => (StatusCode::OK, json_api),
            JsonApiResponse::Err(status_code, errors) => (status_code, JsonApi::errors(errors)),
        };

        response = match response
            .links(vec![Link::SelfReference(&req.uri().to_string())])
            .clone()
            .to_response()
        {
            Ok(response) => response,
            Err(errors) => JsonApi::errors(
                errors
                    .iter()
                    .map(|_error| {
                        JsonApiError::new()
                            .status(999)
                            .title("Document response error")
                            .detail("There was an error parsing the document response.")
                            .finish()
                    })
                    .collect(),
            ),
        };

        let body = serde_json::to_string(&response).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponseBuilder::new(status)
            .content_type("application/vnd.api+json")
            .body(body)))
    }
}
