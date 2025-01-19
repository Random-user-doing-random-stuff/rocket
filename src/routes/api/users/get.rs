use crate::models::user::User;
use rocket::{self, serde::json::Json};

#[get("/api/users")]
pub fn list_users() -> Json<Vec<User>> {
    use crate::controllers::users_controller::list_users;
    let users = list_users();
    Json(users)
}
