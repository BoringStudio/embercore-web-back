use rocket::post;
use rocket::request::{Form, FromForm, FromFormValue};
use rocket_contrib::json::Json;
use serde::Serialize;

/// POST user/auth
#[derive(FromForm, Debug)]
pub struct AuthRequest {
    login: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    user_id: u64,
    access_token: String,
}

#[post("/user/auth", data = "<data>")]
pub fn auth(data: Form<AuthRequest>) -> Json<AuthResponse> {
    println!("Login request: {:?}", data);

    // TODO: get user from database♦

    Json(AuthResponse {
        user_id: 0,
        access_token: String::from("hello world"),
    })
}


/// POST user/register
#[derive(FromForm, Debug)]
pub struct RegistrationRequest {
    email: String,
    password: String,
    name: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum RegistrationResponse {
    Error { error: &'static str },
    Success(AuthResponse),
}

#[post("/user/create", data = "<data>")]
pub fn create(data: Form<RegistrationRequest>) -> Json<RegistrationResponse> {
    if data.name.len() < 3 {
        return Json(RegistrationResponse::Error { error: "Invalid name" });
    }

    Json(RegistrationResponse::Success(AuthResponse {
        user_id: 0,
        access_token: String::from("aaasdasd"),
    }))
}
