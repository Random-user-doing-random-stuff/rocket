#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

mod db;
mod errors;
mod models;
mod schema;
mod utils;

use db::establish_connection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use models::{NewUser, PublicUser, UpdateUser, User};
use rocket::{
    fs::{relative, FileServer},
    serde::json::Json,
};
use schema::users::{self, *};

// Define a static response handler for serving the favicon
static_response_handler! {
    "/favicon.ico" => favicon => "favicon"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // Attach static resources for the favicon
        .attach(static_resources_initializer!(
            "favicon" => "assets/favicon.svg", // Map the favicon resource
        ))
        // Mount the favicon route
        .mount("/", routes![favicon])
        .mount("/landing", FileServer::from(relative!("static/landing")))
        .mount("/tradutor", FileServer::from(relative!("static/camera")))
        .mount("/registar", FileServer::from(relative!("static/signin")))
        // Mount routes for user management
        .mount(
            "/api",
            routes![get_users, get_user, create_user, update_user, delete_user],
        )
}

#[get("/users")]
fn get_users() -> Json<Vec<PublicUser>> {
    let mut conn = establish_connection();
    let users: Vec<User> = users::table
        .load(&mut conn)
        .expect("Error loading users")
        .into();

    let public_users = users
        .iter()
        .map(|user| PublicUser {
            id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            is_active: user.is_active,
            phone_number: user.phone_number.clone(),
            birthdate: user.birthdate,
        })
        .collect();
    Json(public_users)
}

#[get("/users/<user_id>")]
fn get_user(user_id: i32) -> Json<User> {
    let mut conn = establish_connection();
    let user = users::table
        .filter(id.eq(user_id))
        .first(&mut conn)
        .expect("Error");
    Json(user)
}

#[post("/users", data = "<new_user>")]
fn create_user(new_user: Json<NewUser>) -> Json<User> {
    let mut conn = establish_connection();
    let user = diesel::insert_into(users::table)
        .values(&new_user.into_inner())
        .get_result(&mut conn)
        .expect("Error creating user");
    Json(user)
}

#[put("/users/<user_id>", data = "<update_user>")]
fn update_user(user_id: i32, update_user: Json<UpdateUser>) -> Json<User> {
    let mut conn = establish_connection();

    let updated_user = diesel::update(users::table.filter(id.eq(user_id)))
        .set(update_user.into_inner())
        .get_result::<User>(&mut conn)
        .expect("Error updating user");

    Json(updated_user)
}

#[delete("/users/<user_id>")]
fn delete_user(user_id: i32) -> Json<User> {
    let mut conn = establish_connection();
    //let user = diesel::delete(users::table.filter(id.eq(user_id))).get_result(&mut conn).expect("Error deleting user");
    let user = diesel::update(users::table.filter(id.eq(user_id)))
        .set(is_active.eq(false))
        .get_result(&mut conn)
        .expect("Error deleting user");
    Json(user)
}
