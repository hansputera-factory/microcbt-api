use diesel::{prelude::Queryable, Selectable};

use crate::schema::tbl_roles;

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = tbl_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RolesModel {
    pub id: i32,
    pub name: String,

    pub can_insert_student: Option<bool>,
    pub can_update_student: Option<bool>,
    pub can_delete_student: Option<bool>,

    pub can_insert_teacher: Option<bool>,
    pub can_update_teacher: Option<bool>,
    pub can_delete_teacher: Option<bool>,

    pub can_insert_class: Option<bool>,
    pub can_update_class: Option<bool>,
    pub can_delete_class: Option<bool>,

    pub can_insert_semester: Option<bool>,
    pub can_update_semester: Option<bool>,
    pub can_delete_semester: Option<bool>,

    pub is_moderator: Option<bool>,
    pub is_teacher: Option<bool>,
    pub is_student: Option<bool>,
}