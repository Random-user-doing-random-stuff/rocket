use super::schema::{feedbacks, tags, users, word_tags, words};
use chrono;
use diesel_derives::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub is_active: bool,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub birthdate: chrono::NaiveDate,
}
#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Default)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
    pub birthdate: Option<chrono::NaiveDate>,
    pub phone_number: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub birthdate: Option<chrono::NaiveDate>,
}
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct PublicUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
    pub phone_number: Option<String>,
    pub birthdate: chrono::NaiveDate,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable, Debug, Clone)]
#[diesel(table_name = words)]
pub struct Word {
    pub id: i32,
    pub term: String,
    pub video: String,
    pub created_by: i32, // user_id
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Default)]
#[diesel(table_name = words)]
pub struct NewWord {
    pub term: String,
    pub video: String,
    pub created_by: i32,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable, Debug, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Default)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
}

#[derive(Queryable, Debug, Clone, Identifiable)]
#[diesel(table_name = word_tags, primary_key(word_id, tag_id))]
pub struct WordTag {
    pub word_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable, Debug, Clone)]
#[diesel(table_name = feedbacks)]
pub struct Feedback {
    pub id: i32,
    pub word_id: i32,
    pub user_id: i32,
    pub feedback: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone, Default)]
#[diesel(table_name = feedbacks)]
pub struct NewFeedback {
    pub word_id: i32,
    pub user_id: i32,
    pub feedback: String,
}
