use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::UserRole)]
pub enum UserRole {
    User,
    Professor,
    Admin,
}

impl ToSql<crate::schema::sql_types::UserRole, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            UserRole::User => out.write_all(b"user")?,
            UserRole::Professor => out.write_all(b"professor")?,
            UserRole::Admin => out.write_all(b"admin")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::UserRole, Pg> for UserRole {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"user" => Ok(UserRole::User),
            b"professor" => Ok(UserRole::Professor),
            b"admin" => Ok(UserRole::Admin),
            _ => Err("Unrecognized UserRole variant".into()),
        }
    }
}

#[derive(Debug, Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::users)]
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


#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct TestUser {
    pub id: i32,
pub first_name: String,
pub last_name: Option<String>,
pub email: String,
pub role: UserRole,
}