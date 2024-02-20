use serde::{Deserialize, Serialize};
use tonic::Status;
use tonic::{Request, Response};
use crate::utils::idAutoIncrementer::idAutoIncrementer;
use crate::StudCrudMethod::student::CreateStudentRequest;
use crate::StudCrudMethod::student::StudentResponse;
use crate::StudCrudMethod::student::student_server::Student;
use crate::StudCrudMethod::student::ReadStudentRequest;
use crate::Services::StudentSerivce::*;

use self::student::{DeleteStudentRequest, StudentData, UpdateStudentRequest};
pub mod student {
    tonic::include_proto!("student");
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Student_json {
    pub id: u32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Hash, Eq, Clone,Default)]
pub struct StudentServiceImpl {}

#[tonic::async_trait]
impl Student for StudentServiceImpl {
    async fn create(
        &self,
        request: Request<CreateStudentRequest>,
    ) -> Result<Response<StudentResponse>, Status> {
        let req = request.into_inner();
        let data= req.student.unwrap();
        let stud =Student_json{ 
            id: idAutoIncrementer() , 
            name: data.name.clone(), 
            phone: data.phone.clone(), 
            email:data.email.clone(), 
            city: data.city.clone(), 
            address: data.address.clone(), 
            marks: data.marks.clone(), 
            percentage: Some(data.percentage.clone()), 
            grade: Some(data.grade.clone()) 
          
        };
        let reply=StudentResponse{
            status: 200,
            message: create_student_data(stud), 
            student: Some(data)
        };
         Ok(Response::new(reply))
    }

    async fn read(&self, request: Request<ReadStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        let id = request.into_inner().id;
        match read_student_data(id) {
            Ok(data) => {
                if !data.is_empty() {
                    let final_data = StudentData { 
                        id:data[0].id, 
                        name: data[0].name.clone(), 
                        phone: data[0].phone.clone(), 
                        email: data[0].email.clone(), 
                        city: data[0].city.clone(), 
                        address: data[0].address.clone(), 
                        marks: data[0].marks.clone(), 
                        percentage: data[0].percentage.clone().unwrap_or_default(), 
                        grade: data[0].grade.clone().unwrap_or_default() 
                     
                    };
                    let reply = StudentResponse {
                        status: 200,
                        message: "Success".to_string(),
                        student: Some(final_data),
                    };
                    Ok(Response::new(reply))
                } else {
                    let reply = StudentResponse {
                        status: 404,
                        message: "Student not found".to_string(),
                        student: None,
                    };
                    Ok(Response::new(reply))
                }
            }
            Err(err) => {
                let reply = StudentResponse {
                    status: 500,
                    message: format!("Failed to read Student: {}", err),
                    student: None,
                };
                Err(Status::unknown(reply.message))
            }
        }
    }

    async fn update(&self, request: Request<UpdateStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        let req = request.into_inner();
        let id = req.id;
        let data = req.student.unwrap();
        let stud =Student_json{ 
            id: data.id , 
            name: data.name.clone(), 
            phone: data.phone.clone(), 
            email:data.email.clone(), 
            city: data.city.clone(), 
            address: data.address.clone(), 
            marks: data.marks.clone(), 
            percentage: Some(data.percentage.clone()), 
            grade: Some(data.grade.clone()) 
        };
        
        match update_student_data(id, stud) {
            Ok(_) => {
                match read_student_data(id) {
                    Ok(data) => {
                        if let Some(Student) = data.first() {
                            let response = StudentResponse {
                                status: 200,
                                message: "Student updated successfully".to_string(),
                                student: Some(StudentData {
                                     id:Student.id, 
                                     name: Student.name.clone(), 
                                     phone: Student.phone.clone(), 
                                     email: Student.email.clone(), 
                                     city: Student.city.clone(), 
                                     address: Student.address.clone(), 
                                     marks: Student.marks.clone(), 
                                     percentage: Student.percentage.clone().unwrap_or_default(), 
                                     grade: Student.grade.clone().unwrap_or_default()
                                   
                                }),
                            };
                            Ok(Response::new(response))
                        } else {
                            Err(Status::not_found("Student not found"))
                        }
                    }
                    Err(err) => Err(Status::unknown(format!("Failed to read Student: {}", err))),
                }
            }
            Err(err) => Err(Status::unknown(err)),
        }
    }
    

    async fn delete(&self, request: Request<DeleteStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        let id = request.into_inner().id as u64;
    
        match delete_student_data(id) {
            Ok(msg) => {
                            let response = StudentResponse {
                                status: 200,
                                message: "Student deleted successfully".to_string(),
                                student: None
                            };
                            Ok(Response::new(response))
                        },
            Err(err) => Err(Status::unknown(err)),
        }
    }

    
}
