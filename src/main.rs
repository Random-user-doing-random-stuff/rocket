#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

mod controllers;
mod db;
mod models;
mod routes;
mod schema;
mod tera;
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
        .mount("/tera", routes![tera::index, tera::hello, tera::about])
        .mount(
            "/api",
            routes![
                routes::api::users::get::list_users,
                routes::api::users::get_id::get_user,
                routes::api::users::post::new_user
            ],
        )
        // .attach(Template::fairing())
        .attach(Template::custom(|engines| {
            tera::customize(&mut engines.tera)
        }))
}
