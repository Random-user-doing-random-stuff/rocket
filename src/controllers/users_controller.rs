use crate::db::{establish_connection, get_pool};
use crate::models::user::UpdatedUser;
use crate::{
    models::user::{NewUser, User},
    schema::users,
};
use diesel::{QueryDsl, RunQueryDsl};

pub fn get_user(id: i32) -> User {
    let pool = get_pool();
    let conn = &mut pool.get().expect("Failed to get pool connection");
    users::table
        .find(id)
        .first(conn)
        .expect("Error loading user")
}

pub fn list_users() -> Vec<User> {
    let pool = get_pool();
    let conn = &mut pool.get().expect("Failed to get pool connection");
    users::table
        .load::<User>(conn)
        .expect("Error loading users")
}

pub fn create_user(user: NewUser) -> User {
    let pool = get_pool();
    let conn = &mut pool.get().expect("Failed to get pool connection");
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn update_user(id: i32, user: UpdatedUser) -> User {
    let pool = get_pool();
    let conn = &mut pool.get().expect("Failed to get pool connection");
    diesel::update(users::table.find(id))
        .set(&user)
        .get_result(conn)
        .expect("Error updating user")
}
