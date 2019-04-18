//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use crate::schema::students;
use crate::database::Database;
use actix_web::actix::{Message, Handler};
use diesel;

#[allow(unused_imports)] // Throws errors without this import, but throws warning with it :/
use diesel::prelude::*;

mod imports;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub class: String,
    pub phone_number: i32
}

/* Create */
mod create;
pub use create::*;

/* Read */
mod read;
pub use read::*;

/* Update */
mod update;
pub use update::*;

/* Delete */
mod delete;
pub use delete::*;