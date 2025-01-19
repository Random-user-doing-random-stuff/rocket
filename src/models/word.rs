use crate::models::enums::{WordStatus, WordType};
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Debug, Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::words)]
pub struct Word {
    pub id: i32,
    pub term: String,
    pub video: String, // #[max_length = 255] can be validated separately in code
    pub word_type: WordType,
    pub created_by: i32,
    pub status: WordStatus,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = crate::schema::words)]
pub struct NewWord {
    pub term: String,
    pub video: String,
    pub status: WordStatus,
    pub word_type: WordType,
}
