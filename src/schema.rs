// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "word_status"))]
    pub struct WordStatus;
}

diesel::table! {
    comments (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        word_id -> Nullable<Int4>,
        comment -> Text,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Text,
        permissions -> Jsonb,
        active -> Bool,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Text,
        phone_numer -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    word_has_type (type_id, word_id) {
        type_id -> Int4,
        word_id -> Int4,
    }
}

diesel::table! {
    word_tags (word_id, tag_id) {
        word_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    word_type (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WordStatus;

    words (id) {
        id -> Int4,
        word -> Text,
        definition -> Text,
        video_url -> Text,
        created_by -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        status -> Nullable<WordStatus>,
    }
}

diesel::joinable!(comments -> users (user_id));
diesel::joinable!(comments -> words (word_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(word_has_type -> word_type (type_id));
diesel::joinable!(word_has_type -> words (word_id));
diesel::joinable!(word_tags -> tags (tag_id));
diesel::joinable!(word_tags -> words (word_id));
diesel::joinable!(words -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    roles,
    tags,
    user_roles,
    users,
    word_has_type,
    word_tags,
    word_type,
    words,
);
