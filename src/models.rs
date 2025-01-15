use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel_derive_enum::DbEnum;
use diesel_derives::{AsExpression, FromSqlRow, Queryable, SqlType, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use std::io::Write;
use crate::schema::*;
use diesel::sql_types::*;

#[derive(SqlType)]
#[diesel(postgres_type(name = "user_role"))]
pub struct UserRoleType;

#[derive(Debug, FromSqlRow, AsExpression, PartialEq, Eq, Serialize, Deserialize)]
#[diesel(sql_type = UserRoleType)]
pub enum UserRole {
    Admin,
    Professor,
    User,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub role: UserRole,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub last_updated: Option<chrono::NaiveDateTime>,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_token_expiry: Option<chrono::NaiveDateTime>,
    pub is_active: bool,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub email: &'a str,
    pub password: &'a str,
    pub phone_number: Option<&'a str>,
    pub role: UserRole,
}



