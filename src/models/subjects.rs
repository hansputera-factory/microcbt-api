use diesel::prelude::{Identifiable, Queryable};

use crate::schema::tbl_subjects;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = tbl_subjects)]
#[diesel(belongs_to(crate::models::semester::SemesterModel, foreign_key = semester_id))]
#[diesel(belongs_to(crate::models::subject_groups::SubjectGroupsModel, foreign_key = subject_group_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubjectsModel {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub is_active: Option<bool>,
    pub semester_id: i32,
    pub subject_group_id: i32,
}