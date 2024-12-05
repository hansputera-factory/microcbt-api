use std::net::SocketAddr;
use axum::extract::ConnectInfo;
use axum::Json;
use axum::response::IntoResponse;
use validator::{Validate, ValidateIp};
use crate::dto::authentication::login::SignUserPayload;
use crate::entities::authentication::errors::AuthenticationSignValidationErrorResponse;

pub async fn auth_login_route(payload: Json<SignUserPayload>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    if !addr.validate_ip() {
        return Json(AuthenticationSignValidationErrorResponse {
            message: "Client IP invalid".to_string(),
            ..Default::default()
        })
    };

    let validation = payload.validate();
    if validation.is_err() {
        Json(AuthenticationSignValidationErrorResponse {
            message: validation.unwrap_err().to_string(),
            ..Default::default()
        })
    }


}