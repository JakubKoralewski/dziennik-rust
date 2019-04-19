//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use super::*;
use super::imports::*;

/// This is the create handler.
/// 
/// https://github.com/actix/actix-website/blob/master/content/docs/extractors.md#json
pub fn create((request, new_student): (HttpRequest<State>, Json<CreateRequest>)) 
    -> Box<Future<Item = Json<CreateResponse>, Error = actix_web::Error>> 
{
    debug!("Request to create student: {:?}", &new_student);
    /* Add to database */
    request.state().db
        .send(new_student.into_inner())
        .from_err()
        .and_then(|res| {
            info!("Successfully added student");
            Ok(Json(CreateResponse {
                message: "Success!".to_string(),
                student: res.map_err(error::ErrorInternalServerError).ok()
            }))
        })
        .responder()
}

/// id should be set automatically
#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name="students"]
pub struct CreateRequest {
    first_name: String,
    last_name: String,
    class: String,
    phone_number: i32
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub message: String,
    pub student: Option<Student>
}

impl Message for CreateRequest {
    type Result = Result<Student, diesel::result::Error>;
}

impl Handler<CreateRequest> for Database {
    type Result = Result<Student, diesel::result::Error>;

    fn handle(&mut self, msg: CreateRequest, _: &mut Self::Context) -> Self::Result {
        //use crate::schema::students::dsl::*;
        let conn = self.0.get().unwrap();
        println!("Adding student {:?}", &msg);
        diesel::insert_into(students::table).values(&msg).get_result::<Student>(&conn)
    }
}

