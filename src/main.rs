#![allow(unused_imports)]
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
use diesel::{QueryDsl, RunQueryDsl};

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
        .mount("/", routes![index, frustrating_func, mmm])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn frustrating_func() -> Json<Vec<models::TestUser>> {
    use self::schema::users;
    use diesel::prelude::*;

    let conn = &mut db::establish_connection();

    /*     let new_user = models::User {
           id: 0, // Automatically assigned by SERIAL
           first_name: "Alice".to_string(),
           last_name: Some("Smith".to_string()),
           email: "alice.smith@example.com".to_string(),
           password: "hashedpassword".to_string(),
           phone_number: Some("+987654321".to_string()),
           role: models::UserRole::Professor,
           created_at: None,
           last_updated: None,
           last_login: None,
           password_reset_token: None,
           password_reset_token_expiry: None,
           is_active: true,
       };

       diesel::insert_into(users::table)
           .values(&new_user)
           .execute(conn)
           .expect("Error inserting user");
    */
    let results = users::table
        .select((
            users::id,
            users::first_name,
            users::last_name,
            users::email,
            users::role,
        ))
        .load::<models::TestUser>(conn)
        .expect("Error loading users");

    Json(results)
}

#[post("/testi")]
fn mmm() -> Json<User> {
    use self::schema::users;
    use diesel::prelude::*;

    let conn = &mut db::establish_connection();

    let new_user = models::User {
        id: 0, // Automatically assigned by SERIAL
        first_name: "Alice".to_string(),
        last_name: Some("Smith".to_string()),
        email: "alice.smith@example.com".to_string(),
        password: "hashedpassword".to_string(),
        phone_number: Some("+987654321".to_string()),
        role: models::UserRole::Professor,
        created_at: None,
        last_updated: None,
        last_login: None,
        password_reset_token: None,
        password_reset_token_expiry: None,
        is_active: true,
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error inserting user");
    Json(user)
}
