use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Timestamp};
use crate::database;
use crate::models::auth_logs::AuthLogsModel;
use crate::schema::tbl_auth_logs;
use crate::state::CONFIG_STATES;

pub fn check_logs(user_id: &i32, user_agent: &String, device: &String, browser: &String, ip: &String) -> bool {
    let connection = &mut database::connection::get_connection();
    let connection = match connection {
        Ok(connection) => connection,
        Err(_) => return false,
    };

    let now_timestamp = chrono::offset::Utc::now();
    let results = tbl_auth_logs::table
        .select(AuthLogsModel::as_select())
        .filter(
            tbl_auth_logs::user_id.eq(user_id).and(
                tbl_auth_logs::created_at.between(
                    sql::<Nullable<Timestamp>>((now_timestamp - chrono::Duration::hours(1)).naive_utc().to_string().as_str()),
                    sql::<Nullable<Timestamp>>((now_timestamp + chrono::Duration::hours(1)).naive_utc().to_string().as_str()),
                )
            )
        ).get_results(connection);

    let results: Vec<AuthLogsModel> = match results {
        Ok(results) => results,
        Err(_) => return false,
    };

    if results.len() >= CONFIG_STATES.auth.limit as usize {
        false
    } else {
        let first_record = results.first();

        // Checking if first record is not null
        if !first_record.is_none() {
            let record = first_record.expect("First record exist");
            // Checking if we block the request has different user agent or devices
            if (!record.client_user_agent.eq(user_agent) && CONFIG_STATES.auth.block_different_useragent) ||
                (!record.client_device.eq(device) && CONFIG_STATES.auth.block_different_device) {
                    false
                } else {
                    // Check how much differences in tolerant
                    let different_log_len = results.iter().filter(|auth_log| {
                        !auth_log.client_browser.eq(browser) ||
                        !auth_log.client_ip.eq(ip) ||
                        !auth_log.client_user_agent.eq(user_agent) ||
                        !auth_log.client_device.eq(device)
                    }).count();
            
                    if different_log_len > CONFIG_STATES.auth.tolerance_different as usize {
                        false
                    } else {
                        true
                    }
                }
        } else {
            false
        }
    }
}