use diesel::{Insertable, Queryable};
use diesel_derives::AsChangeset;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct Comment {
    pub id: i32,
    pub user_id: Option<i32>,
    pub word_id: Option<i32>,
    pub comment: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, FromForm, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment {
    pub id: i32,
    pub user_id: i32,
    pub word_id: i32,
    pub comment: String,
}
