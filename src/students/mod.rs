extern crate serde_derive;
extern crate actix_web;

use actix_web::{
    Json,
    HttpRequest,
    Path
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    first_name: String,
    last_name: String,
    id: Option<u32>,
    class: String,
    phone_number: u32
}

#[derive(Serialize)]
pub struct CreateResponse {
    message: String,
    new_student: Student
}

#[derive(Serialize, Deserialize)]
pub struct StudentUpdate {
    first_name: Option<String>,
    last_name: Option<String>,
    class: Option<String>,
    phone_number: Option<u32>
}

#[derive(Deserialize, Serialize)]
pub struct UpdateResponse {
    message: String,
    student: Option<StudentUpdate> // TODO: Get new student from database and respond with that.
}

#[derive(Serialize)]
pub struct DeleteResponse {
    message: String,
}

/// https://github.com/actix/actix-website/blob/master/content/docs/extractors.md#json
pub fn create(new_student: Json<Student>) -> Json<CreateResponse> {
    println!("Waiting for extraction of json. {:?}", &new_student);
    Json(CreateResponse{message: "Success!".to_string(), new_student: new_student.into_inner()})
}

pub fn read(_request: &HttpRequest) -> Json<Vec<Student>> {
    //! Was helpful: https://docs.rs/actix-web/0.6.7/actix_web/struct.Json.html
    Json(vec![
        Student{
            id: Some(0),
            first_name: "Jakub".to_string(),
            last_name: "Koralewski".to_string(),
            class: "XD".to_string(),
            phone_number: 123456789,
        }
    ])
}

pub fn update((id, updated_student): (Path<u32>, Json<StudentUpdate>)) -> Json<UpdateResponse> {
    Json(UpdateResponse{message: format!("Updated student with id: {:?}.", id), student: Some(updated_student.into_inner())})
}

pub fn delete(id: Path<u32>) -> Json<DeleteResponse> {
    Json(DeleteResponse{message: format!("Deleted student with id: {:?}.", id).to_string()})
}

