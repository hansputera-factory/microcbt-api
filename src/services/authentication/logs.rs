use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Timestamp;
use crate::database;
use crate::models::auth_logs::AuthLogsModel;
use crate::schema::tbl_auth_logs;
use crate::state::CONFIG_STATES;

pub fn check_logs(user_id: i32) -> bool {
    let connection = &mut database::connection::get_connection();
    let connection = match connection {
        Ok(connection) => connection,
        Err(_) => return false,
    };

    let now_timestamp = chrono::offset::Utc::now();
    let results = tbl_auth_logs::table.filter(tbl_auth_logs::user_id.eq(user_id))
        .filter(
            tbl_auth_logs::created_at.between(
                sql::<Timestamp>((now_timestamp - chrono::Duration::hours(1)).naive_utc().to_string().as_str()),
                sql::<Timestamp>((now_timestamp + chrono::Duration::hours(1)).naive_utc().to_string().as_str())
            )
        ).select(AuthLogsModel::as_select()).get_results(connection);
    let results: Vec<AuthLogsModel> = match results {
        Ok(results) => results,
        Err(_) => return false,
    };

    if results.len() >= CONFIG_STATES.auth.limit as usize {
        false
    }
}