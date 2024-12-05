use diesel::{prelude::{Associations, Identifiable, Queryable}, Selectable};

use crate::schema::tbl_class;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug, Associations)]
#[diesel(table_name = tbl_class)]
#[diesel(belongs_to(crate::models::semester::SemesterModel, foreign_key = semester_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassModel {
    pub id: i32,
    pub name: String,
    pub grade: i32,
    pub students_count: i32,
    pub major_id: i32,
    pub semester_id: i32,
}
