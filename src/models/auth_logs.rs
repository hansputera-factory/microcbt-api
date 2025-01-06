use diesel::{Identifiable, Queryable, Selectable};

use crate::schema::tbl_auth_logs;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = tbl_auth_logs)]
#[diesel(belongs_to(crate::models::UsersModel, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthLogsModel {
    pub id: i32,

    pub client_ip: String,
    pub client_user_agent: String,
    pub client_device: String,
    pub client_os: String,
    pub client_browser: String,

    pub user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>
}