use axum::Router;
use axum::routing::post;
use crate::routes::authentication::login::auth_login_route;

pub fn create_authentication_routes() -> Router {
    Router::new()
        .route("/login", post(auth_login_route))
}