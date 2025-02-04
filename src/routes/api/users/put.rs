use crate::{
    controllers::users_controller::get_user,
    models::user::{UpdatedUser, User},
};
use rocket::{self, form::Form, http::hyper::header::FROM, serde::json::Json};

#[put("/api/users/<user_id>", data = "<updated_user>")]
pub fn update_user(user_id: i32, updated_user: Form<UpdatedUser>) -> Json<User> {
    // Filter out empty fields
    let user_data = updated_user.into_inner();

    // Filter out empty fields
    let mut fields_to_update = Vec::new();

    if let Some(ref username) = user_data.username {
        fields_to_update.push(("username", username));
    }
    if let Some(ref first_name) = user_data.first_name {
        fields_to_update.push(("first_name", first_name));
    }
    if let Some(ref last_name) = user_data.last_name {
        fields_to_update.push(("last_name", last_name));
    }
    if let Some(ref email) = user_data.email {
        fields_to_update.push(("email", email));
    }
    if let Some(ref password) = user_data.password {
        fields_to_update.push(("password", password));
    }
    if let Some(ref phone_number) = user_data.phone_number {
        fields_to_update.push(("phone_number", phone_number));
    }

    // Handle the is_active field
    if let Some(is_active) = user_data.is_active {
        fields_to_update.push(("is_active", &is_active.to_string()));
    }

    // Now you can send `fields_to_update` to the database or other API endpoint.
    // Example: update_user_in_db(user_id, fields_to_update);

    // For now, we'll redirect the user to a success page or something appropriate
    Json(get_user(user_id))
}
