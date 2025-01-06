use serde::Serialize;

#[derive(Serialize)]
pub enum SignErrorCodes {
    ClientIPInvalid = 5005,
    LoginValidationError = 5006,
    MissingUa = 6001,
}