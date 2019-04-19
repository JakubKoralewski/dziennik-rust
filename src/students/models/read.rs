//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use super::*;
use super::imports::*;

pub fn read(request: &HttpRequest<State>) 
    -> Box<Future<Item = Json<Vec<Student>>, Error = actix_web::Error>> 
{
    debug!("Request to read all students.");
    request.state().db
        .send(ReadRequest{})
        .from_err()
        .and_then(|res| res.map(Json).map_err(error::ErrorInternalServerError))
        .responder()
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
        students.order(id).load::<Student>(&conn)
    }
}