//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ ConnectionManager, Pool };
use actix_web::actix::{Actor, SyncContext};

pub fn pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Error creating PostgreSQL connection pool!")
}

pub struct Database(pub Pool<ConnectionManager<PgConnection>>);

//unsafe impl Send for Database {}

impl Actor for Database {
    type Context = SyncContext<Self>;
}
