use crate::schema::devices;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = devices )]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub id: i32,
    pub owner: String,
    pub device_name: String,
    pub device_type: String,
    pub device_token: String,
    pub os_version: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
