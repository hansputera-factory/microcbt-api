use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use diesel::prelude::*;
use crate::{database, schema::tbl_users};
use crate::services::authentication::structs::JwtClaims;
use crate::state::CONFIG_STATES;

pub fn verify_token(token: String) -> bool {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some("authentication".to_string());
    validation.set_issuer(&["MicroCBT"]);
    validation.set_required_spec_claims(&["exp", "sub", "iss", "iat", "id", "username", "role"]);

    let token_data = match decode::<JwtClaims>(token.as_str(), &DecodingKey::from_secret(CONFIG_STATES.jwt_secret.as_ref()), &validation) {
        Ok(token_data) => token_data,
        Err(_) => return false,
    };

    let connection = &mut database::connection::get_connection().unwrap();
    let count_user: i64 = tbl_users::table.filter(
        tbl_users::id.eq(&token_data.claims.id).and(
            tbl_users::username.eq(&token_data.claims.username).and(
                tbl_users::is_active.eq(true),
            )
        ),
    ).count().get_result(connection).expect("couldn't check user existence");

    count_user > 0
}