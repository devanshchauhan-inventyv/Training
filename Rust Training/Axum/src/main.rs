mod SimpleProgram;
use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;
mod composition_and_static_routing;
use crate::composition_and_static_routing::*;
mod database_integration;
use crate::database_integration::*;
use crate::SimpleProgram::*;
mod generate_random_number;
use crate::generate_random_number::*;
mod json_crud;
use crate::json_crud::*;
mod configuration;
use crate::configuration::*;
mod user_agent;
use user_agent::user_agent;


#[tokio::main]
async fn main() {
    //    match tokio::spawn(async {
    //        SimpleProgam().await;
    //     }).await{
    //         Ok(val)=>val,
    //         Err(err)=>{
    //             println!("{}",err);
    //             return;
    //         }

    //     };

    //    match tokio::spawn(async {
    //         genrate_random_number().await;
    //     }).await{
    //         Ok(val)=>val,
    //         Err(err)=>{
    //             println!("{}",err);
    //             return;
    //         }

    //     };

    // compostion_and_static_routing().await;

    // database_connect().await;

    // start_crud_server().await;

    // config_example();

    user_agent().await;

    
}
