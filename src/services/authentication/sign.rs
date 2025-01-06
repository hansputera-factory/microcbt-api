use argon2::Argon2;
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use password_hash::PasswordHash;
use crate::{database, models::users::UsersModel, schema::{tbl_users, tbl_roles}};
use crate::dto::authentication::login::SignUserPayload;
use crate::services::authentication::structs::JwtClaims;
use crate::state::CONFIG_STATES;

use super::logs::check_logs;

pub fn sign_in_token(payload: SignUserPayload, user_agent: &String, ip: &String) -> Result<String, String> {
    let connection = &mut database::connection::get_connection()
        .expect("Query connection");

    let result = tbl_users::table.select(
        UsersModel::as_select()
    ).filter(tbl_users::username.eq(payload.username))
        .get_result::<UsersModel>(connection).expect("Query error");

    let is_allow_logged = check_logs(&result.id, &user_agent, &payload.client_device, &payload.client_device, &ip);
    if !is_allow_logged {
        return Err("This user isnt allowed to logged in".to_owned());
    }

    let hashed_password = PasswordHash::new(&result.password).expect("Hashing successful");
    hashed_password.verify_password(&[&Argon2::default()], payload.password).expect("Verify successful");

    if result.is_active.eq(&Some(false)) {
        return Err("User is inactive".to_owned());
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