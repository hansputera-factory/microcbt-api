use std::fmt::Error;
use argon2::Argon2;
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use password_hash::PasswordHash;
use crate::{database, models::users::UsersModel, schema::{tbl_users, tbl_roles}};
use crate::dto::authentication::login::SignUserPayload;
use crate::services::authentication::structs::JwtClaims;
use crate::state::CONFIG_STATES;

pub fn sign_in_token(payload: SignUserPayload) -> Result<String, Error> {
    let connection = &mut database::connection::get_connection()
        .expect("Query connection");

    let result = tbl_users::table.filter(
        tbl_users::username.eq(payload.username)
    ).select(UsersModel::as_select()).get_result(connection).expect("Query successful");

    let hashed_password = PasswordHash::new(&result.password).expect("Hashing successful");
    hashed_password.verify_password(&[&Argon2::default()], payload.password).expect("Verify successful");

    if !result.is_active {
        Err("User inactive".into_string()).expect("User inactive");
    }

    let role = tbl_roles::table.filter(tbl_roles::id.eq(result.id)).select(tbl_roles::name).get_result(connection).expect("Query successful");
    let token = encode(&Header::default(), &JwtClaims {
        iat: chrono::offset::Utc::now().timestamp() as usize,
        iss: "MicroCBT".to_string(),
        exp: (chrono::offset::Utc::now() + chrono::Duration::hours(2)).timestamp() as usize,
        id: result.id,
        sub: "authentication".to_string(),
        username: result.username,
        name: result.name,
        role,
    }, &EncodingKey::from_secret(CONFIG_STATES.jwt_secret.as_ref())).expect("Signing successful");

    Ok(token)
}