extern crate actix_web;
extern crate listenfd;
#[macro_use] extern crate serde_derive;

use listenfd::ListenFd;
use actix_web::{
    server, 
    App,
    http::Method,
};

mod students;
mod login;

fn main() {
    const IP_PORT: &str = "127.0.0.1:3000";
    println!("Listening on {}", &IP_PORT);
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .prefix("/api")
            .scope(
                "/students",
                |students_scope| {
                    students_scope
                        .resource("", |r| {
                            r.method(Method::POST).with(students::create);
                            r.method(Method::GET).f(students::read);
                        })
                        .resource("/{id}", |r| {
                            r.method(Method::PUT).with(students::update);
                            r.method(Method::DELETE).with(students::delete);
                        })
                }
            ).scope(
                "/login",
                |login_scope| {
                    login_scope
                        .resource("", |r| {
                                r.method(Method::POST).with(login::login);
                        })
                }            
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(&IP_PORT).unwrap()
    };

    server.run();
} 



