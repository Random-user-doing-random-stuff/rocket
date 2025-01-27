use chrono;
use diesel::{Insertable, Queryable};
use diesel_derives::AsChangeset;
use serde::Serialize;
#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub permissions: i16, // Unsigned 16-bit integer
    pub is_active: bool,
    pub password_reset_token: Option<String>,
    pub password_reset_token_expires_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, FromForm, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub permissions: Option<i16>, // Unsigned 16-bit integer
    pub is_active: Option<bool>,
}

#[derive(Debug, Insertable, FromForm, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdatedUser {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub permissions: Option<i16>, // Unsigned 16-bit integer
    pub is_active: Option<bool>,
}
