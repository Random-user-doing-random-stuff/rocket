use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel_derives::{AsExpression, Insertable, Queryable};
use serde_json::Value;
use std::io::Write;

use crate::schema::roles; // Assuming `roles` is the table schema imported from `schema.rs`

/// Implement FromSql for JsonValue
impl FromSql<Jsonb, Pg> for Role {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let json: Value = serde_json::from_slice(bytes.as_bytes())
            .map_err(|e| format!("Error deserializing jsonb: {}", e))?;
        Ok(json)
    }
}

/// Implement ToSql for JsonValue
impl ToSql<Jsonb, Pg> for Role {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let json_string = serde_json::to_string(&self.0)
            .map_err(|e| format!("Error serializing jsonb: {}", e))?;
        out.write_all(json_string.as_bytes())?;
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub permissions: serde_json::Value,
    pub active: bool,
}
