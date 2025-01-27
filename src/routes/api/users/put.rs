use crate::models::user::{UpdatedUser, User};
use rocket::{self, form::Form, serde::json::Json};

#[put("/api/users/<user_id>", data = "<updated_user>")]
pub fn update_user(user_id: i32, updated_user: Form<UpdatedUser>) -> Json<User> {
    use crate::controllers::users_controller::update_user;
    let user = update_user(user_id, updated_user.into_inner());
    Json(user)
}
