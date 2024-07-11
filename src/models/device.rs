use crate::schema::devices;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
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

// Custom validation function for the `enabled` field
fn validate_enabled(enabled: &bool) -> Result<(), ValidationError> {
    if *enabled {
        Ok(())
    } else {
        Err(ValidationError::new("enabled_must_be_true"))
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate, Debug)]
#[diesel(table_name = devices )]
pub struct CreateDevice {
    // @TODO: needs UUID validate
    pub owner: String,
    pub device_name: String,
    pub device_type: String,
    pub device_token: String,
    pub os_version: String,
    #[validate(custom(function = "validate_enabled"))]
    pub enabled: bool,
}
