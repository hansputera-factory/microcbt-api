use diesel::{prelude::{Insertable, Queryable}, Selectable};

use crate::schema::tbl_settings;

#[derive(Queryable, Selectable, PartialEq, Debug, Insertable)]
#[diesel(table_name = tbl_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SettingsModel {
    pub id: i32,
    pub name: String,
    pub school_name: String,
    pub school_address: String,
    pub village: String,
    pub district: String,
    pub city: String,
    pub province: String,
    pub postal_code: Option<String>,
    pub fax: Option<String>,
    pub registered_id: String,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub headmaster_name: String,
    pub headmaster_id: Option<String>,

    pub signature_photo_url: Option<String>,
    pub school_logo_url: Option<String>,
    pub app_logo_url: Option<String>,
}