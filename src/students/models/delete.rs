//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use super::*;
use super::imports::*;

/// This is the delete handler
pub fn delete((request, id): (HttpRequest<State>, Path<i32>)) -> HttpResponse {
    // Diesel's `delete` method returns number of deleted rows, so we can check if we deleted something
    let num_of_del_rows = request.state().db.send(DeleteRequest{id: id.clone()}).wait().expect("Resolving future").expect("Obtaining value");
    
    if num_of_del_rows > 0 {
        HttpResponse::Ok().json(DeleteResponse{message: format!("Deleted student with id: {:?}.", id).to_string()})
    } else {
        HttpResponse::BadRequest().json(DeleteResponse{message: format!("Something went wrong! I only know I deleted {} rows though.", num_of_del_rows).to_string()})
    }
    
}

#[derive(Serialize, Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}

impl Message for DeleteRequest {
    type Result = Result<usize, diesel::result::Error>;
}

impl Handler<DeleteRequest> for Database {
    type Result = Result<usize, diesel::result::Error>;

    fn handle(&mut self, msg: DeleteRequest, _: &mut Self::Context) -> Self::Result {
        use crate::schema::students::dsl::*;
        let conn = self.0.get().unwrap();
        diesel::delete(students.filter(id.eq(msg.id))).execute(&conn)
    }
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub message: String,
}