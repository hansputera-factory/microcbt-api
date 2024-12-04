use diesel::prelude::{Identifiable, Insertable, Queryable};

use crate::schema::tbl_users;

#[derive(Queryable, Identifiable, Insertable, PartialEq, Debug)]
#[diesel(table_name = tbl_users)]
#[diesel(belongs_to(crate::models::role::RoleModel, foreign_key = role_id))]
#[diesel(belongs_to(crate::models::semester::SemesterModel, foreign_key = semester_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UsersModel {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_active: Option<bool>,
    pub role_id: i32,
    pub semester_id: i32,
}