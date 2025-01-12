use std::net::SocketAddr;
use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use validator::Validate;
use crate::dto::authentication::login::SignUserPayload;
use crate::entities::authentication::errors::AuthenticationSignValidationErrorResponse;
use crate::entities::authentication::success::{AuthenticationSignTokenResponse, AuthenticationSignTokenSuccess};
use crate::{enums, services};

#[axum_macros::debug_handler]
pub async fn auth_login_route(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<SignUserPayload>,
) -> impl IntoResponse {
    // Validate client IP
    if !is_valid_ipv4(&addr) {
        return (
            StatusCode::BAD_REQUEST,
            Json::<AuthenticationSignValidationErrorResponse>(AuthenticationSignValidationErrorResponse {
                message: "Client IP invalid".to_string(),
                error_code: enums::error_codes::SignErrorCodes::ClientIPInvalid,
                ..Default::default()
            }),
        );
    }

    // Validate payload
    if let Err(validation_error) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json::<AuthenticationSignValidationErrorResponse>(AuthenticationSignValidationErrorResponse {
                message: validation_error.to_string(),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error_code: enums::error_codes::SignErrorCodes::LoginValidationError,
            }),
        );
    }

    // Extract the `User-Agent` header
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if user_agent.len() < 10 {
        return (
            StatusCode::BAD_REQUEST,
            Json::<AuthenticationSignValidationErrorResponse>(AuthenticationSignValidationErrorResponse {
                message: "Missing UA".to_string(),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error_code: enums::error_codes::SignErrorCodes::MissingUa,
            }),
        );
    }

    // Generate authentication token
    match services::authentication::sign::sign_in_token(
        payload,
        &user_agent.to_string(),
        &addr.to_string(),
    ) {
        Ok(token) => (
            StatusCode::OK,
            Json::<AuthenticationSignValidationErrorResponse>(AuthenticationSignValidationErrorResponse {
                message: "Login successful".to_string(),
                status_code: StatusCode::OK.as_u16(),
                data: Some(AuthenticationSignTokenSuccess { token }),
            }),
        ),
        Err(token_error) => (
            StatusCode::UNAUTHORIZED,
            Json::<AuthenticationSignValidationErrorResponse>(AuthenticationSignValidationErrorResponse {
                message: token_error.to_string(),
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
                error_code: enums::error_codes::SignErrorCodes::LoginValidationError,
            }),
        ),
    }
}

// Helper function to validate IPv4 addresses
fn is_valid_ipv4(addr: &SocketAddr) -> bool {
    match addr {
        SocketAddr::V4(_) => true,
        SocketAddr::V6(_) => false,
    }
}
