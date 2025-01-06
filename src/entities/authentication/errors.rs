use axum::http::StatusCode;
use serde::Serialize;

use crate::enums;

#[derive(Serialize)]
pub struct AuthenticationSignValidationErrorResponse {
    pub message: String,
    pub status_code: u16,
    pub error_code: enums::error_codes::SignErrorCodes,
}

impl Default for AuthenticationSignValidationErrorResponse {
    fn default() -> AuthenticationSignValidationErrorResponse {
        AuthenticationSignValidationErrorResponse {
            message: "Sign payload is invalid".to_string(),
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            error_code: enums::error_codes::SignErrorCodes::LoginValidationError,
        }
    }
}