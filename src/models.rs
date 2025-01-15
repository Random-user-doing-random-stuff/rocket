#![allow(unused_imports)]
use diesel::prelude::*;
use diesel::pg::PgValue;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use serde::{Serialize, Deserialize};
use crate::schema::*;
use crate::types::UserRoleType;  // Import UserRoleType from types.rs
use diesel::pg::Pg;
use std::io::Write;
use diesel::sql_types::Text;
// Enum representing the UserRole
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Professor,
    User,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "Admin",
            UserRole::Professor => "Professor",
            UserRole::User => "User",
        }
    }
}

// Implement ToSql for UserRole
impl ToSql<UserRoleType, Pg> for UserRole {
    fn to_sql<'a>(&'a self, out: &mut Output<'a, '_, Pg>) -> serialize::Result {
        out.write_all(self.as_str().as_bytes())?;  // Write bytes to output
        Ok(IsNull::No)
    }
}

// Implement FromSql for UserRole
impl FromSql<UserRoleType, Pg> for UserRole {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let raw_bytes = bytes.as_bytes();
        match raw_bytes {
            b"Admin" => Ok(UserRole::Admin),
            b"Professor" => Ok(UserRole::Professor),
            b"User" => Ok(UserRole::User),
            _ => Err("Unrecognized variant".into()),
        }
    }
}

// Implement Expression for UserRole
impl Expression for UserRole {
    type SqlType = UserRoleType;  // Make sure UserRole is associated with UserRoleType
}

#[derive(Debug, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name:  Option<String>, 
    pub email:  String,
    pub password:   String,
    pub phone_number:   Option<String>,
    pub role:   UserRole,
    pub created_at:  Option<chrono::NaiveDateTime>,
    pub last_updated:  Option<chrono::NaiveDateTime>,
    pub last_login:  Option<chrono::NaiveDateTime>,
    pub password_reset_token:  Option<String>,
    pub password_reset_token_expiry:  Option<chrono::NaiveDateTime>,
    pub is_active:  bool,
}

// NewUser struct that is Insertable
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub email: &'a str,
    pub password: &'a str,
    pub phone_number: Option<&'a str>,
    pub role: UserRole,
}

impl<'a> NewUser<'a> {
    pub fn new(
        first_name: &'a str,
        last_name: Option<&'a str>,
        email: &'a str,
        password: &'a str,
        phone_number: Option<&'a str>,
        role: UserRole,
    ) -> Self {
        NewUser {
            first_name,
            last_name,
            email,
            password,
            phone_number,
            role,
        }
    }
}
