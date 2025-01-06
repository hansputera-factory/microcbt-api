use serde::Serialize;

#[derive(Serialize)]
pub struct AuthenticationSignTokenSuccess {
    pub token: String,
}

#[derive(Serialize)]
pub struct AuthenticationSignTokenResponse {
    pub data: AuthenticationSignTokenSuccess,
    pub status_code: u16,
    pub message: String,
}
