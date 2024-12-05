use axum::{http::header, Router};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer, sensitive_headers::SetSensitiveHeadersLayer};

use crate::routes;

pub async fn create_app() -> Router {
    Router::new()
        .merge(routes::index::create_routes())
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing_core::Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing_core::Level::INFO))
        )
        .layer(
            SetSensitiveHeadersLayer::new(std::iter::once(
                header::AUTHORIZATION
            ))
        )
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static("x-request-id")))
        .layer(CorsLayer::permissive())
}
