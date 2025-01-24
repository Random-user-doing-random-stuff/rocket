use diesel_derives::{Identifiable, Queryable};

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::word_type)]
pub struct WordType {
    pub id: i32,
    pub type_: String,
}
