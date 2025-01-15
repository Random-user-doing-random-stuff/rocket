#![recursion_limit = "512"]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

mod db;
mod errors;
mod models;
mod schema;
mod types;
mod utils;

use crate::models::User;
use diesel::RunQueryDsl;

use rocket::{
    fs::{relative, FileServer},
    serde::json::Json,
};

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
        .mount("/", routes![index, frustrating_func])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/test")]
fn frustrating_func() -> Json<User> {
    use crate::schema::users;
    use diesel::RunQueryDsl;
    let mut conn = db::establish_connection();
    let mut new_user = models::NewUser::new(
        "John",
        Some("Doe"),
        "john.doe@email.com",
        "password123",
        None,
        models::UserRole::Admin,
    );
    let resul = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .unwrap();
    Json(resul)
}
