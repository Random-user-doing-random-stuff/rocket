use diesel::{Insertable, Queryable};
use diesel_derives::AsChangeset;
use serde::Serialize;

use crate::models::enums::UserRole;
#[derive(Debug, Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub role: UserRole,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub last_updated: Option<chrono::NaiveDateTime>,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_token_expiry: Option<chrono::NaiveDateTime>,
    pub is_active: bool,
}

#[derive(Debug, Insertable, Serialize, FromForm, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub role: Option<UserRole>,
}
