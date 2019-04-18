//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use super::*;
use super::imports::*;

pub fn read(request: &HttpRequest<State>) -> Json<Vec<Student>> {
    //! Was helpful: https://docs.rs/actix-web/0.6.7/actix_web/struct.Json.html
    let all_students = request.state().db
        .send(ReadRequest{})
        .wait()
        .expect("Future didn't resolve")
        .expect("Error reading students from database");
    
    Json(all_students)
}

pub struct ReadRequest{}

impl Message for ReadRequest {
    type Result = Result<Vec<Student>, diesel::result::Error>;
}

impl Handler<ReadRequest> for Database {
    type Result = Result<Vec<Student>, diesel::result::Error>;

    fn handle(&mut self, _msg: ReadRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::students::dsl::*;
        let conn = self.0.get().unwrap();
        println!("Reading all students.");
        students.order(id).load::<Student>(&conn)
    }
}