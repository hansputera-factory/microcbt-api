use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JwtClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub id: i32,
    pub iss: String,
    pub username: String,
    pub name: String,
    pub role: String
}
