use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Serialize, FromFormField)]
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
            WordStatus::Draft => out.write_all(b"draft")?,
            WordStatus::Review => out.write_all(b"review")?,
            WordStatus::Approved => out.write_all(b"approved")?,
            WordStatus::Archived => out.write_all(b"archvied")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::WordStatus, Pg> for WordStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"draft" => Ok(WordStatus::Draft),
            b"review" => Ok(WordStatus::Review),
            b"approved" => Ok(WordStatus::Approved),
            b"archvied" => Ok(WordStatus::Archived),
            _ => Err("Unrecognized WordStatus variant".into()),
        }
    }
}
