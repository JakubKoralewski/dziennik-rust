extern crate serde_derive;
extern crate actix_web;

use actix_web::{
    Json,
};

#[derive(Serialize, Deserialize)]
pub struct LoginCredentials {
    login: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    message: String,
}

pub fn login(credentials: Json<LoginCredentials>) -> Json<LoginResponse> {
    Json(LoginResponse{message: format!("Your login is {} and password is {}. Get h4x0red.", credentials.login, credentials.password)})
}