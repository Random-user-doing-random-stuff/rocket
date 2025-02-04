use rocket::form::Form;
use rocket::serde::json::Json;

use crate::controllers::users_controller::create_user;
use crate::db::establish_connection;
use crate::models::user::{NewUser, User};

#[rocket::post("/users", data = "<new_user>")]
pub fn new_user(new_user: Form<NewUser>) -> Json<User> {
    let user = create_user(new_user.into_inner());
    //let user = User::create(new_user.into_inner());
    Json(user)
}
