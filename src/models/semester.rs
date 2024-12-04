use diesel::prelude::{Identifiable, Insertable, Queryable};

use crate::schema::tbl_semester;

#[derive(Queryable, Identifiable, PartialEq, Debug, Insertable)]
#[diesel(table_name = tbl_semester)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SemesterModel {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub is_active: Option<bool>,
}