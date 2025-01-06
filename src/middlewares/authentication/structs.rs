use serde::Serialize;

#[derive(Serialize)]
pub struct AuthenticationError {
    pub message: String,
    pub status_code: u16,
}