use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(
    Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Serialize, FromFormField, Deserialize,
)]
#[diesel(sql_type = crate::schema::sql_types::WordStatus)]
pub enum WordStatus {
    Draft,
    Review,
    Approved,
    Archived,
}

impl ToSql<crate::schema::sql_types::WordStatus, Pg> for WordStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            WordStatus::Approved => out.write_all(b"approved")?,
            WordStatus::Archived => out.write_all(b"archived")?,
            WordStatus::Draft => out.write_all(b"draft")?,
            WordStatus::Review => out.write_all(b"review")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::WordStatus, Pg> for WordStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"approved" => Ok(WordStatus::Approved),
            b"archived" => Ok(WordStatus::Archived),
            b"draft" => Ok(WordStatus::Draft),
            b"review" => Ok(WordStatus::Review),
            _ => Err("Unrecognized UserRole variant".into()),
        }
    }
}
