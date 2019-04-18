//! MIT License
//! Copyright (c) 2019 Jakub Koralewski

use super::*;
use super::imports::*;

pub fn update((request, id, updated_student): (HttpRequest<State>, Path<i32>, Json<UpdateRequest>)) -> HttpResponse {
    let updated_student = request.state().db
        .send(UpdateStudent{id: id.clone(), fields: updated_student.into_inner()})
        .wait()
        .expect("Future didn't resolve");
    
    if let Ok(student) = updated_student {
        HttpResponse::Ok().json(
            UpdateResponse{ 
                message: format!("Updated student with id: {:?}.", id),
                student: Some(student),
            }
        )
    } else {
        HttpResponse::BadRequest().json(
            UpdateResponse { 
                message: format!("Something went wrong. User with id of {} may not exist.", id),
                student: None,
            }
        )
    }
}

// use of undeclared type or module `student_fieldss`
/// 
/// https://www.reddit.com/r/rust/comments/9qeldl/diesel_orm_asking_for_modules_that_do_not_exist/
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name="students"]
pub struct UpdateRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub class: Option<String>,
    pub phone_number: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStudent {
    pub id: i32,
    pub fields: UpdateRequest,
}

impl Message for UpdateStudent {
    type Result = Result<Student, diesel::result::Error>;
}

impl Handler<UpdateStudent> for Database {
    type Result = Result<Student, diesel::result::Error>;

    fn handle(&mut self, msg: UpdateStudent, _: &mut Self::Context) -> Self::Result {
        use crate::schema::students::dsl::*;
        let conn = self.0.get().unwrap();
        diesel::update(students.filter(id.eq(msg.id))).set(msg.fields).get_result::<Student>(&conn)
    }
}

#[derive(Deserialize, Serialize)]
pub struct UpdateResponse {
    pub message: String,
    pub student: Option<Student>
}