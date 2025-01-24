use crate::models::enums::WordStatus;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Debug, Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::words)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub definition: String,
    pub video_url: String,
    pub created_by: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub status: Option<WordStatus>,
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = crate::schema::words)]
pub struct NewWord {
    pub word: String,
    pub definition: String,
    pub video_url: String,
}
