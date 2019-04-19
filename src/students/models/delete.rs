//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use super::*;
use super::imports::*;

use sentry::{Hub, Level};
use sentry_actix::ActixWebHubExt;

/// This is the delete handler
pub fn delete((request, id): (HttpRequest<State>, Path<i32>)) 
    -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> 
{
    // Diesel's `delete` method returns number of deleted rows, so we can check if we deleted something
    debug!("Request to delete student with id of {}.", id.as_ref());
    
    request.state().db
        .send(DeleteRequest{id: id.clone()})
        .from_err()
        .and_then(move |num_of_del_rows| {
            let num_of_del_rows = num_of_del_rows.expect("Database error when deleting student");
            if num_of_del_rows > 0 {
                info!("Student with id of {} successfully deleted.", id);
                Ok(HttpResponse::Ok()
                    .json(DeleteResponse {
                        message: format!("Deleted student with id: {:?}.", id).to_string()
                    })
                )
            } else {
                let message = format!("Student with id of `{}` not found or something because I found {} rows.", id, &num_of_del_rows);
                info!("{}", &message);
                let hub = Hub::from_request(&request);

                hub.capture_message(message.as_str(), Level::Error);
                
                Ok(HttpResponse::BadRequest()
                    .json(DeleteResponse{message})
                )
            }
        }).responder()
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