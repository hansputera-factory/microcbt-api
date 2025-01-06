use std::net::SocketAddr;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use validator::{Validate, ValidateIp};
use crate::dto::authentication::login::SignUserPayload;
use crate::entities::authentication::errors::AuthenticationSignValidationErrorResponse;
use crate::entities::authentication::success::{AuthenticationSignTokenResponse, AuthenticationSignTokenSuccess};
use crate::{enums, services};

#[axum::]
pub async fn auth_login_route(payload: Json<SignUserPayload>, TypedHeader(user_agent): TypedHeader<UserAgent>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    if !addr.validate_ip() {
        Json(AuthenticationSignValidationErrorResponse {
            message: "Client IP invalid".to_string(),
            error_code: enums::error_codes::SignErrorCodes::ClientIPInvalid,
            ..Default::default()
        }).into_response()
    } else {
        let error_payload = payload.validate();
        if error_payload.is_err() {
            Json(AuthenticationSignValidationErrorResponse {
                message: error_payload.unwrap_err().to_string(),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                error_code: enums::error_codes::SignErrorCodes::LoginValidationError,
            }).into_response()
        } else {
            if user_agent.as_str().len() < 10 {
                Json(AuthenticationSignValidationErrorResponse {
                    message: "Missing UA".to_string(),
                    status_code: StatusCode::UNAUTHORIZED.as_u16(),
                    error_code: enums::error_codes::SignErrorCodes::MissingUa,
                }).into_response()
            } else {
                let token = services::authentication::sign::sign_in_token(payload.0, &user_agent.to_string(), &addr.to_string());
                if token.is_err() {
                    Json(AuthenticationSignValidationErrorResponse {
                        message: token.unwrap_err(),
                        status_code: StatusCode::UNAUTHORIZED.as_u16(),
                        error_code: enums::error_codes::SignErrorCodes::LoginValidationError,
                    }).into_response()
                } else {
                    Json(AuthenticationSignTokenResponse {
                        message: "login successfuly".to_string(),
                        status_code: StatusCode::OK.as_u16(),
                        data: AuthenticationSignTokenSuccess {
                            token: token.unwrap(),
                        },
                    }).into_response()
                }
            }
        }
    }
}