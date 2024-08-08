use crate::{core::validations::boolean_validator::validate_enabled, schema::devices};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = devices )]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub id: i32,
    pub owner: i32,
    pub device_name: String,
    pub device_type: String,
    pub device_token: String,
    pub os_version: String,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable, Serialize, Deserialize, Validate, Debug)]
#[diesel(table_name = devices )]
pub struct CreateDevice {
    // @TODO: needs UUID validate
    pub owner: i32,
    pub device_name: String,
    pub device_type: String,
    pub device_token: String,
    pub os_version: String,
    #[validate(custom(function = "validate_enabled"))]
    pub enabled: bool,
}
