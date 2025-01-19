#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

mod controllers;
mod db;
mod models;
mod routes;
mod schema;
mod types;
mod utils;

use crate::models::enums::UserRole;
use crate::models::user::{NewUser, User};
use rocket::{
    form::Form,
    fs::{relative, FileServer},
    serde::json::Json,
};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

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
        .mount(
            "/api",
            routes![
                routes::api::users::get::list_users,
                routes::api::users::get_id::get_user,
                routes::api::users::post::new_user
            ],
        )
        .attach(Template::fairing())
}

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Rocket + Bootstrap Project");
    Template::render("index", &context)
}
