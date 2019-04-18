//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use super::*;
use super::imports::*;

/// This is the create handler.
/// 
/// https://github.com/actix/actix-website/blob/master/content/docs/extractors.md#json
pub fn create((request, new_student): (HttpRequest<State>, Json<CreateRequest>)) -> Json<CreateResponse> {
    /* Add to database */

    let new_student = request.state().db
        .send(new_student.into_inner())
        .wait()
        .expect("Future didn't resolve")
        .expect("Error adding student in database");

    println!("{:?}", new_student);

    /* Create response */
    Json(CreateResponse{message: "Success!".to_string(), new_student: Some(new_student)})
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
    pub new_student: Option<Student>
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

