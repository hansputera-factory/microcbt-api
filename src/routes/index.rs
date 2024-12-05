use axum::{routing::get, Router};
use crate::routes::authentication::route::create_authentication_routes;
use crate::routes::protected::route::create_protected_routes;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(index_route))
        .nest("/protected", create_protected_routes())
        .nest("/auth", create_authentication_routes())
}

// Index Route
async fn index_route() -> String {
    "Hello World".to_string()
}