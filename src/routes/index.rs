use axum::{routing::get, Router};

pub fn create_route() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> String {
    "Hello World".to_string()
}
