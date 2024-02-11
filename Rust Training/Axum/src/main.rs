mod SimpleProgram;
use std::net::SocketAddr;

use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
mod composition_and_static_routing;
use crate::composition_and_static_routing::*;
mod database_integration;
use crate::database_integration::*;

use crate::SimpleProgram::*;
mod generate_random_number;
use crate::generate_random_number::*;

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

    database_connect().await;
}
