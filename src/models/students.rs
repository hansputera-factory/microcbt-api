use diesel::prelude::{Associations, Identifiable, Queryable};

use crate::schema::tbl_students;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = tbl_students)]
#[diesel(belongs_to(crate::models::semester::SemesterModel, foreign_key = semester_id))]
#[diesel(belongs_to(crate::models::class::ClassModel, foreign_key = class_id))]
#[diesel(belongs_to(crate::models::users::UsersModel, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentsModel {
    pub id: i32,
    pub name: String,
    pub national_student_id: String,
    pub school_student_id: Option<String>,

    pub class_id: i32,
    pub semester_id: i32,
    pub user_id: i32,
}