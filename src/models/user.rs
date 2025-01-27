use diesel::{Insertable, Queryable};
use diesel_derives::AsChangeset;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
#[derive(Debug, Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub phone_numer: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, FromForm, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub phone_numer: Option<String>,
}
