use diesel::prelude::*;
use crate::models::user::User;
use crate::schema::devices;

#[derive(Queryable, Selectable)]
#[diesel(table_name = devices )]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Device {
    pub id: Option<String>,
    pub owner: String,
    pub device_name: String,
    pub device_type: String,
    pub device_token: String,
    pub os_version: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}