use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthenticationSignValidationErrorResponse {
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: i64,
}

impl Default for AuthenticationSignValidationErrorResponse {
    fn default() -> AuthenticationSignValidationErrorResponse {
        AuthenticationSignValidationErrorResponse {
            message: "Sign payload is invalid".to_string(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: 1001,
        }
    }
}