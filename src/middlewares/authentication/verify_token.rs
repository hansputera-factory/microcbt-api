use axum::body::Body;
use axum::extract::Request;
use axum::http::{Response, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use crate::middlewares::authentication::structs::AuthenticationError;
use crate::services;

pub async fn verify_token_middleware(mut request: Request, next: Next) -> Result<Response<Body>, AuthenticationError> {
    let bearer_token_header = request.headers_mut().get(
        AUTHORIZATION
    );

    let bearer_token_header = match bearer_token_header {
        Some(token) => token.to_str().map_err(|_| AuthenticationError {
            message: "forbidden access".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => return Err(AuthenticationError {
            message: "forbidden access".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })
    };

    let mut header = bearer_token_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());

    let _bearer = match _bearer {
        Some(token) => token,
        None => return Err(AuthenticationError {
            message: "missing token identifier".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })
    };


    let token = match token {
        Some(token) => token,
        None => return Err(AuthenticationError {
            message: "missing token value".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })
    };

    if !services::authentication::verify::verify_token(token.to_string()) {
        return Err(AuthenticationError {
            message: "token invalid".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })
    };

    Ok(next.run(request).await)
}