use actix_web::{get, http::StatusCode, web, App, HttpServer, Responder};
use json_api::{JsonApi, JsonApiError, Resource, ResourceTrait};
use json_api_actix::JsonApiResponse;
use std::sync::Mutex;

#[derive(Debug, Clone, Resource)]
struct Articles {
    #[json_api(id)]
    id: usize,
    title: String,
    #[json_api(to_one)]
    author: Author,
}

#[derive(Debug, Clone, Resource)]
struct Author {
    #[json_api(id)]
    id: usize,
    name: String,
}

struct ArticleState {
    articles: Mutex<Vec<Articles>>,
}

#[get("/articles")]
async fn get_articles(data: web::Data<ArticleState>) -> impl Responder {
    let resources = data.articles.lock().unwrap();
    JsonApiResponse::Ok(JsonApi::collection(resources.to_vec()))
}

#[get("/articles/{id}")]
async fn get_article(data: web::Data<ArticleState>, path: web::Path<(usize,)>) -> impl Responder {
    let resources = data.articles.lock().unwrap();
    match resources
        .iter()
        .find(|article| article.id == path.0)
        .cloned()
    {
        Some(article) => JsonApiResponse::Ok(JsonApi::data(article)),
        None => JsonApiResponse::Err(
            StatusCode::NOT_FOUND,
            vec![JsonApiError::new()
                .status(404)
                .code(1)
                .title("Resource not found")
                .detail("The requested resource could not be found.")
                .finish()],
        ),
    }
}

#[get("/articles/{id}/author")]
async fn get_article_author(
    data: web::Data<ArticleState>,
    path: web::Path<(usize,)>,
) -> impl Responder {
    let resources = data.articles.lock().unwrap();
    match resources
        .iter()
        .find(|article| article.id == path.0)
        .cloned()
    {
        Some(article) => JsonApiResponse::Ok(JsonApi::data(article.author)),
        None => JsonApiResponse::Err(
            StatusCode::NOT_FOUND,
            vec![JsonApiError::new()
                .status(404)
                .code(1)
                .title("Resource not found")
                .detail("The requested resource could not be found.")
                .finish()],
        ),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut map = Vec::new();
    map.push(Articles {
        id: 1,
        title: "Rails is Omakase".to_string(),
        author: Author {
            id: 1,
            name: "Zach LeBlanc".to_string(),
        },
    });
    map.push(Articles {
        id: 2,
        title: "Hello".to_string(),
        author: Author {
            id: 2,
            name: "Not me`".to_string(),
        },
    });
    let counter = web::Data::new(ArticleState {
        articles: Mutex::new(map),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(get_article)
            .service(get_articles)
            .service(get_article_author)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
