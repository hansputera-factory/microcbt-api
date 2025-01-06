use axum::response::IntoResponse;
use axum::{middleware, Json, Router};
use axum::routing::get;
use crate::middlewares::authentication::verify_token::verify_token_middleware;
use crate::routes::protected::index::index_protected;

pub fn create_protected_routes() -> Router {
    Router::new()
        .route("/", get(index_protected))
        .layer(middleware::from_fn(move |req, next| async {
            match verify_token_middleware(req, next).await {
                Ok(res) => res,
                Err(err) => Json(err).into_response(),
            }
        }))
}
