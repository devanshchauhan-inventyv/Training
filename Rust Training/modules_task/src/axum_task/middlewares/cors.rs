use std::future::{Future, IntoFuture};

use axum::{http::{header, HeaderValue, Method}, response::Response};
use tower_http::cors::CorsLayer;


pub fn create_cors_layer()-> CorsLayer{
    let allowed_headers = vec![
        header::CONTENT_TYPE
    ];
CorsLayer::new()
.allow_headers(allowed_headers)
    .allow_origin("http://127.123.5.8:3002".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET,Method::POST])
}