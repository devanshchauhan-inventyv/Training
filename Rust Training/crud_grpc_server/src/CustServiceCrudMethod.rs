use serde::{Deserialize, Serialize};
use tonic::Request;
use tonic::Response;
use tonic::Status;
use crate::utils::idAutoIncrementer::idAutoIncrementer;
use crate::CustServiceCrudMethod::customerService::customer_service_server::CustomerService;
use crate::CustServiceCrudMethod::customerService::CreateCustomerServiceRequest;
use crate::CustServiceCrudMethod::customerService::CustomerServiceResponse;
use crate::CustServiceCrudMethod::customerService::ReadCustomerServiceRequest;
use crate::CustServiceCrudMethod::customerService::CustomerServiceData;
use crate::CustServiceCrudMethod::customerService::UpdateCustomerServiceRequest;
use crate::CustServiceCrudMethod::customerService::DeleteCustomerServiceRequest;
use crate::Services::CustomerService::*;

pub mod customerService {
    tonic::include_proto!("customer_Service");
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Customer_support {
    pub id: u32,
    pub name: String,
    pub skills: Vec<String>,
    pub status: String,
    pub language: String,
}

#[derive(Clone,Default,Debug)]
pub struct CustomerServiceImpl {}

#[tonic::async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn create(
        &self,
        request: Request<CreateCustomerServiceRequest>,
    ) -> Result<Response<CustomerServiceResponse>, Status> {
        let req = request.into_inner();
        let data = req.customer_service.unwrap();
        let serv =Customer_support{ 
            id: idAutoIncrementer(), 
            name: data.name.clone(), 
            skills: data.skills.clone(),
             status: data.status.clone(),
              language: data.language.clone(),  
           
        };
        let reply=CustomerServiceResponse{
            status: 200,
            message: create_customerService_data(serv), 
            customer_service:  Some(data)
        };
         Ok(Response::new(reply))
    }

    async fn read(&self, request: Request<ReadCustomerServiceRequest>) -> Result<Response<CustomerServiceResponse>, Status> {
        let id = request.into_inner().id;
        match read_customerService_data(id) {
            Ok(data) => {
                if !data.is_empty() {
                    let final_data = CustomerServiceData {
                       id: data[0].id,
                       name: data[0].name.clone(),
                       skills: data[0].skills.clone(),
                       status: data[0].status.clone(),
                       language: data[0].language.clone(),
                    };
                    let reply = CustomerServiceResponse {
                        status: 200,
                        message: "Success".to_string(),
                        customer_service: Some(final_data),
                    };
                    Ok(Response::new(reply))
                } else {
                    let reply = CustomerServiceResponse {
                        status: 404,
                        message: "CustomerService not found".to_string(),
                        customer_service: None,
                    };
                    Ok(Response::new(reply))
                }
            }
            Err(err) => {
                let reply = CustomerServiceResponse {
                    status: 500,
                    message: format!("Failed to read CustomerService: {}", err),
                    customer_service: None,
                };
                Err(Status::unknown(reply.message))
            }
        }
    }

    async fn update(&self, request: Request<UpdateCustomerServiceRequest>) -> Result<Response<CustomerServiceResponse>, Status> {
        let req = request.into_inner();
        let id = req.id;
        let data = req.customer_service.unwrap();
        let emp =Customer_support{ 
           id:data.id.clone(),
           name: data.name.clone(),
           skills: data.skills.clone(),
           status: data.status.clone(),
           language: data.language.clone(),
        };
        
        match update_customerService_data(id, emp) {
            Ok(_) => {
                match read_customerService_data(id) {
                    Ok(data) => {
                        if let Some(CustomerService) = data.first() {
                            let response = CustomerServiceResponse {
                                status: 200,
                                message: "CustomerService updated successfully".to_string(),
                                customer_service: Some(CustomerServiceData {
                                  id:CustomerService.id.clone(),
                                   name: CustomerService.name.clone(),
                                   skills: CustomerService.skills.clone(),
                                   status: CustomerService.status.clone(),
                                   language: CustomerService.language.clone(),
                                }),
                            };
                            Ok(Response::new(response))
                        } else {
                            Err(Status::not_found("Customer Service not found"))
                        }
                    }
                    Err(err) => Err(Status::unknown(format!("Failed to read CustomerService: {}", err))),
                }
            }
            Err(err) => Err(Status::unknown(err)),
        }
    }
    

    async fn delete(&self, request: Request<DeleteCustomerServiceRequest>) -> Result<Response<CustomerServiceResponse>, Status> {
        let id = request.into_inner().id;
        match delete_customerService_data(id) {
            Ok(msg) => {     
                            let response = CustomerServiceResponse {
                                status: 200,
                                message: "CustomerService deleted successfully".to_string(),
                                customer_service: None
                            };
                            Ok(Response::new(response))
                        },
            Err(err) => Err(Status::unknown(err)),
        }
    }
}
    
