use crate::schema::roles::dsl::*;
use crate::{db::establish_connection, models::role::Role, schema::roles};
use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

pub fn list_roles() -> Vec<serde_json::Value> {
    let conn = &mut establish_connection();
    let troll = diesel::select::<serde_json::Value>(roles::dsl::permissions).get_results(conn);
    troll
}
