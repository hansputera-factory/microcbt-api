use axum::Router;
use axum::routing::get;
use crate::middlewares::authentication::verify_token::verify_token_middleware;
use crate::routes::protected::index::index_protected;

pub fn create_protected_routes() -> Router {
    Router::new()
        .route("/", get(index_protected))
        .layer(verify_token_middleware)
}
