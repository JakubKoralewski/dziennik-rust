//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

extern crate actix_web;
extern crate listenfd;
#[macro_use] extern crate serde_derive;
extern crate sentry_actix;
#[macro_use] extern crate diesel;

use sentry_actix::SentryMiddleware;
use listenfd::ListenFd;
use actix_web::{
    server, 
    App,
    http::Method,
    middleware,
    error,
    HttpRequest,
    HttpResponse,
    dev::JsonConfig,
    actix::{SyncArbiter, Addr, System},
};
use std::env;
use dotenv::dotenv;

mod students;
mod login;
mod schema;
mod database;

#[derive(Deserialize, Serialize)]
struct JsonError {
    message: String,
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest<State>) -> error::Error {
    let description = JsonError{message: format!("{}", err)};
    error::InternalError::from_response(
        err, HttpResponse::BadRequest().json(description)
    ).into()
}

pub struct State {
    pub db: Addr<database::Database>
}

fn main() {
    /* Environment variables */
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug,info,warn");

    /* Sentry */
    let _sentry;
    if let Ok(dsn) = env::var("SENTRY_DSN") {
        _sentry = sentry::init(dsn);
        sentry::integrations::panic::register_panic_handler();
    }

    const IP_PORT: &str = "127.0.0.1:3000";
    println!("Listening on {}", &IP_PORT);

    /* Setup autoreload */
    let mut listenfd = ListenFd::from_env();

    /* Database */
    let sys = System::new("dziennik");
    let pool = database::pool();
    let addr = SyncArbiter::start(12, move || database::Database(pool.clone()));

    /* Start server */
    let mut server = server::new(move || {
        App::with_state(State {
            db: addr.clone()
        })
            .middleware(SentryMiddleware::new())
            .middleware(middleware::Logger::default())
            .prefix("/api")
            .scope(
                "/students",
                |students_scope| {
                    students_scope
                        .resource("", |r| {
                            r.method(Method::POST).with_config(students::create, |cfg: &mut ((((), JsonConfig<State>),))| {
                                (cfg.0).1.error_handler(&json_error_handler);
                            });
                            r.method(Method::GET).f(students::read);
                        })
                        .resource("/{id}", |r| {
                            r.method(Method::PUT).with_config(students::update, |cfg| {
                                (cfg.0).2.error_handler(&json_error_handler);
                            });
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

    // For auto-reload
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(&IP_PORT).unwrap()
    };

    server.run();
    sys.run();
} 



