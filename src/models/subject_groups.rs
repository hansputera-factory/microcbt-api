use diesel::prelude::{Identifiable, Queryable};

use crate::schema::tbl_subject_groups;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = tbl_subject_groups)]
#[diesel(belongs_to(crate::models::semester::SemesterModel, foreign_key = semester_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubjectGroupsModel {
    pub id: i32,
    pub name: String,
    pub is_universal: Option<bool>,
    pub semester_id: i32,
}