use diesel::{Insertable, Queryable};
use serde::Serialize;
#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::schema::tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = crate::schema::tags)]
pub struct NewTag {
    pub name: String,
}
