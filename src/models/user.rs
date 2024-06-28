use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users )] // Use the imported `users` module
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub created_at: String,
    pub updated_at: String,
}