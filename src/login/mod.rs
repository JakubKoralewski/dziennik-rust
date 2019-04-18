//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

extern crate serde_derive;
extern crate actix_web;

use actix_web::actix::{Message, Handler};
use diesel;
#[allow(unused_imports)] // Throws errors without this import, but throws warning with it :/
use diesel::prelude::*;
use futures::future::Future;

use crate::database::Database;
//use crate::schema::users;
use crate::State;

use actix_web::{
    Json,
    HttpResponse,
    HttpRequest,
};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    login: String,
    password: String,
}

impl Message for LoginRequest {
    type Result = Result<Vec<User>, diesel::result::Error>;
}

impl Handler<LoginRequest> for Database {
    type Result = Result<Vec<User>, diesel::result::Error>;

    fn handle(&mut self, msg: LoginRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = self.0.get().unwrap();
        let req_login = msg.login.trim().to_owned();
        let req_password = msg.password.trim().to_owned();
        users.filter(login.eq(req_login).and(password.eq(req_password))).load(&conn)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    message: String,
}

#[derive(Queryable)]
#[allow(dead_code)] // The properties are needed for Diesel querying
pub struct User {
    id: i32,
    login: String,
    password: String,
}

/// This is the login handler
/// 
/// Returns empty response body. If found such user returns Response 200 OK. Else 400.
pub fn login((request, credentials): (HttpRequest<State>, Json<LoginRequest>)) -> HttpResponse {
    let found_users = request.state().db
        .send(credentials.into_inner())
        .wait()
        .expect("Future didn't resolve")
        .expect("Error trying to get user from database")
        .len();

    if found_users == 0 {
        HttpResponse::BadRequest().finish()
    } else {
        HttpResponse::Ok().finish()
    }
    
}