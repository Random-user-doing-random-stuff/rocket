use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::Serialize;
use std::io::Write;


#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Serialize, FromFormField)]
#[diesel(sql_type = crate::schema::sql_types::UserRole)]
pub enum UserRole {
    User,
    Professor,
    Admin,
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::WordStatus)]
pub enum WordStatus {
    Pending,
    Approved,
    Rejected,
    Inactive,
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::FeedbackType)]
pub enum FeedbackType {
    Suggestion,
    BugReport,
    Comment,
}

#[derive(Debug, AsExpression, FromSqlRow, Clone, PartialEq, Eq, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::WordType)]
pub enum WordType {
    Verb,
    Noun,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Other,
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

impl ToSql<crate::schema::sql_types::WordStatus, Pg> for WordStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            WordStatus::Pending => out.write_all(b"pending")?,
            WordStatus::Approved => out.write_all(b"approved")?,
            WordStatus::Rejected => out.write_all(b"rejected")?,
            WordStatus::Inactive => out.write_all(b"inactive")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::WordStatus, Pg> for WordStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(WordStatus::Pending),
            b"approved" => Ok(WordStatus::Approved),
            b"rejected" => Ok(WordStatus::Rejected),
            b"inactive" => Ok(WordStatus::Inactive),
            _ => Err("Unrecognized WordStatus variant".into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::FeedbackType, Pg> for FeedbackType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            FeedbackType::Suggestion => out.write_all(b"suggestion")?,
            FeedbackType::BugReport => out.write_all(b"bug_report")?,
            FeedbackType::Comment => out.write_all(b"comment")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::FeedbackType, Pg> for FeedbackType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"suggestion" => Ok(FeedbackType::Suggestion),
            b"bug_report" => Ok(FeedbackType::BugReport),
            b"comment" => Ok(FeedbackType::Comment),
            _ => Err("Unrecognized FeedbackType variant".into()),
        }
    }
}

impl ToSql<crate::schema::sql_types::WordType, Pg> for WordType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            WordType::Verb => out.write_all(b"verb")?,
            WordType::Noun => out.write_all(b"noun")?,
            WordType::Adjective => out.write_all(b"adjective")?,
            WordType::Adverb => out.write_all(b"adverb")?,
            WordType::Pronoun => out.write_all(b"pronoun")?,
            WordType::Preposition => out.write_all(b"preposition")?,
            WordType::Conjunction => out.write_all(b"conjunction")?,
            WordType::Interjection => out.write_all(b"interjection")?,
            WordType::Other => out.write_all(b"other")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::WordType, Pg> for WordType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"verb" => Ok(WordType::Verb),
            b"noun" => Ok(WordType::Noun),
            b"adjective" => Ok(WordType::Adjective),
            b"adverb" => Ok(WordType::Adverb),
            b"pronoun" => Ok(WordType::Pronoun),
            b"preposition" => Ok(WordType::Preposition),
            b"conjunction" => Ok(WordType::Conjunction),
            b"interjection" => Ok(WordType::Interjection),
            b"other" => Ok(WordType::Other),
            _ => Err("Unrecognized WordType variant".into()),
        }
    }
}
