mod Services;
mod EmpCrudMethods;
mod utils;
mod StudCrudMethod;
mod CustServiceCrudMethod;
use std::net::SocketAddr;
use tonic::transport::Server;
use CustServiceCrudMethod::CustomerServiceImpl;
use EmpCrudMethods::EmployeeServiceImpl;
use StudCrudMethod::{student::student_server::StudentServer, StudentServiceImpl};
use crate::EmpCrudMethods::employee::employee_server::EmployeeServer;
use crate::CustServiceCrudMethod::customerService::customer_service_server::CustomerServiceServer;






#[tokio::main]
async fn main() {
    let addr:SocketAddr="127.0.0.1:3000".parse().unwrap();
    let empservice=EmployeeServiceImpl::default();
    let studservice=StudentServiceImpl::default();
    let custservice=CustomerServiceImpl::default();
    Server::builder().add_service(EmployeeServer::new(empservice))
    .add_service(StudentServer::new(studservice))
    .add_service(CustomerServiceServer::new(custservice))
    .serve(addr).await.unwrap();

}
