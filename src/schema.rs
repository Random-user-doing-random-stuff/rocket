// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "feedback_type"))]
    pub struct FeedbackType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "word_status"))]
    pub struct WordStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "word_type"))]
    pub struct WordType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FeedbackType;

    feedbacks (id) {
        id -> Int4,
        word_id -> Int4,
        user_id -> Int4,
        feedback -> Text,
        feedback_type -> FeedbackType,
        created_at -> Timestamp,
        response -> Nullable<Text>,
        response_timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
        phone_number -> Nullable<Varchar>,
        role -> UserRole,
        created_at -> Nullable<Timestamp>,
        last_updated -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
        password_reset_token -> Nullable<Varchar>,
        password_reset_token_expiry -> Nullable<Timestamp>,
        is_active -> Bool,
    }
}

diesel::table! {
    word_tags (word_id, tag_id) {
        word_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WordType;
    use super::sql_types::WordStatus;

    words (id) {
        id -> Int4,
        term -> Varchar,
        #[max_length = 255]
        video -> Varchar,
        word_type -> WordType,
        created_by -> Int4,
        status -> WordStatus,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(feedbacks -> users (user_id));
diesel::joinable!(feedbacks -> words (word_id));
diesel::joinable!(word_tags -> tags (tag_id));
diesel::joinable!(word_tags -> words (word_id));
diesel::joinable!(words -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    feedbacks,
    tags,
    users,
    word_tags,
    words,
);
