use crate::models::user::User;
use rocket::{self, serde::json::Json};

#[get("/api/users/<user_id>")]
pub fn get_user(user_id: i32) -> Json<User> {
    use crate::controllers::users_controller::get_user;
    let users = get_user(user_id);
    Json(users)
}
