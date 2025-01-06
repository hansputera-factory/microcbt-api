use diesel::prelude::{Identifiable, Queryable};

use crate::schema::tbl_semester;

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = tbl_semester)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SemesterModel {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>
}