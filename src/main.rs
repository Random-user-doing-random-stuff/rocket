#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

mod db;
mod errors;
mod models;
mod schema;
mod utils;

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
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
        .mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
