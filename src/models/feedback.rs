use diesel::{Insertable, Queryable};
use serde::Serialize;

use crate::models::enums::FeedbackType;
#[derive(Debug, Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::feedbacks)]
pub struct Feedback {
    pub id: i32,
    pub word_id: i32,
    pub user_id: i32,
    pub feedback: String,
    pub feedback_type: FeedbackType,
    pub created_at: chrono::NaiveDateTime,
    pub response: Option<String>,
    pub response_timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = crate::schema::feedbacks)]
pub struct NewFeedback {
    pub word_id: i32,
    pub user_id: i32,
    pub feedback: String,
    pub feedback_type: FeedbackType,
}
