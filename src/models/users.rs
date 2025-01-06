use diesel::{prelude::{Identifiable, Queryable}, Selectable};

use crate::schema::tbl_users;

#[derive(Selectable, Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = tbl_users)]
#[diesel(belongs_to(crate::models::role::RoleModel, foreign_key = role_id))]
#[diesel(belongs_to(crate::models::semester::SemesterModel, foreign_key = semester_id))]
#[diesel(belongs_to(crate::models::teachers::TeachersModel))]
#[diesel(belongs_to(crate::models::students::StudentsModel))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UsersModel {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub is_active: Option<bool>,
    pub role_id: i32,
    pub semester_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>
}