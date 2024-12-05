use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct SignUserPayload {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min=3, max = 50))]
    pub password: String
}
