use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use crate::{utils::idAutoIncrementer::idAutoIncrementer, Services::EmployeeService::{create_employee_data, delete_employee_data, read_employee_data, update_employee_data}};
use self::employee::{employee_server::{Employee,EmployeeServer}, CreateEmployeeRequest, DeleteEmployeeRequest, EmployeeData, EmployeeResponse, ReadEmployeeRequest, UpdateEmployeeRequest};


pub mod employee {
    tonic::include_proto!("employee");
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Hash, Eq, Clone)]
pub struct Employee_json {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub skills: Vec<String>,
    pub position: Option<String>,
    #[serde(rename = "experience(y)")]
    pub experience_y: Option<u32>,
}

#[derive(Clone,Default,Debug)]
pub struct EmployeeServiceImpl {}

#[tonic::async_trait]
impl Employee for EmployeeServiceImpl {
    async fn create(
        &self,
        request: Request<CreateEmployeeRequest>,
    ) -> Result<Response<EmployeeResponse>, Status> {
        let req = request.into_inner();
        let data = req.employee.unwrap();
        let emp =Employee_json{ 
            id: idAutoIncrementer(), 
            name: data.name.clone(), 
            age: data.age, 
            skills: data.skills.clone(), 
            position: data.position.clone(), 
            experience_y: data.experience_y 
        };
        let reply=EmployeeResponse{
            status: 200,
            message: create_employee_data(emp), 
            employee:  Some(data)
        };
         Ok(Response::new(reply))
    }

    async fn read(&self, request: Request<ReadEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        let id = request.into_inner().id;
        match read_employee_data(id) {
            Ok(data) => {
                if !data.is_empty() {
                    let final_data = EmployeeData {
                        id: data[0].id,
                        name: data[0].name.clone(),
                        age: data[0].age,
                        skills: data[0].skills.clone(),
                        position: data[0].position.clone(),
                        experience_y: data[0].experience_y,
                    };
                    let reply = EmployeeResponse {
                        status: 200,
                        message: "Success".to_string(),
                        employee: Some(final_data),
                    };
                    Ok(Response::new(reply))
                } else {
                    let reply = EmployeeResponse {
                        status: 404,
                        message: "Employee not found".to_string(),
                        employee: None,
                    };
                    Ok(Response::new(reply))
                }
            }
            Err(err) => {
                let reply = EmployeeResponse {
                    status: 500,
                    message: format!("Failed to read employee: {}", err),
                    employee: None,
                };
                Err(Status::unknown(reply.message))
            }
        }
    }

    async fn update(&self, request: Request<UpdateEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        let req = request.into_inner();
        let id = req.id;
        let data = req.employee.unwrap();
        let emp =Employee_json{ 
            id: data.id, 
            name: data.name.clone(), 
            age: data.age, 
            skills: data.skills.clone(), 
            position: data.position.clone(), 
            experience_y: data.experience_y 
        };
        
        match update_employee_data(id, emp) {
            Ok(_) => {
                match read_employee_data(id) {
                    Ok(data) => {
                        if let Some(employee) = data.first() {
                            let response = EmployeeResponse {
                                status: 200,
                                message: "Employee updated successfully".to_string(),
                                employee: Some(EmployeeData {
                                    id: employee.id,
                                    name: employee.name.clone(),
                                    age: employee.age,
                                    skills: employee.skills.clone(),
                                    position: employee.position.clone(),
                                    experience_y: employee.experience_y,
                                }),
                            };
                            Ok(Response::new(response))
                        } else {
                            Err(Status::not_found("Employee not found"))
                        }
                    }
                    Err(err) => Err(Status::unknown(format!("Failed to read employee: {}", err))),
                }
            }
            Err(err) => Err(Status::unknown(err)),
        }
    }
    

    async fn delete(&self, request: Request<DeleteEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        let id = request.into_inner().id;
        match delete_employee_data(id) {
            Ok(msg) => {     
                            let response = EmployeeResponse {
                                status: 200,
                                message: "Employee deleted successfully".to_string(),
                                employee: None
                            };
                            Ok(Response::new(response))
                        },
            Err(err) => Err(Status::unknown(err)),
        }
    }
}
    
