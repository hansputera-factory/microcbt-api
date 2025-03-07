use diesel::{prelude::{Associations, Identifiable, Queryable}, Selectable};

use crate::schema::tbl_majors;

#[derive(Queryable, Identifiable, Selectable, Associations, PartialEq, Debug)]
#[diesel(table_name = tbl_majors)]
#[diesel(belongs_to(crate::models::semester::SemesterModel, foreign_key = semester_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MajorsModel {
    pub id: i32,
    pub name: String,
    pub code: String,

    pub semester_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>
}