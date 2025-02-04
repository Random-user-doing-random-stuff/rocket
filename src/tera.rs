use std::collections::HashMap;

use rocket::{form::Context, response::Redirect};
use rocket::{Request, State};

use rocket_dyn_templates::{context, tera::Tera, Template};

use crate::controllers::users_controller::get_user;
use crate::models::{permissions::UserPermissions, user::UpdatedUser};

fn get_permissions_list() -> Vec<HashMap<String, String>> {
    UserPermissions::all()
        .iter()
        .map(|perm| {
            let mut map = HashMap::new();
            map.insert("name".to_string(), format!("{}", perm.get_name())); // Permission name
            map.insert("value".to_string(), perm.bits().to_string()); // Bit value
            map
        })
        .collect()
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/tera", hello(name = "Your Name")))
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "tera/index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}

#[get("/admin")]
pub fn admin() -> Template {
    Template::render(
        "tera/new-user",
        context! {
            permissions: get_permissions_list()
        },
    )
}
#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "tera/about.html",
        context! {
            title: "About",
        },
    )
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "tera/error/404",
        context! {
            uri: req.uri()
        },
    )
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template(
        "tera/about.html",
        r#"
        {% extends "tera/base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#,
    )
    .expect("valid Tera template");
}

#[get("/update_user/<user_id>")]
pub fn update_user_form(user_id: i32) -> Template {
    Template::render(
        "tera/update_user",
        context! {
            user: get_user(user_id)
        },
    )
}
