use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub word_id: i32,
    pub comment: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment {
    pub word_id: i32,
    pub user_id: i32,
    pub comment: String,
}
