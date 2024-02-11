use std::{collections::binary_heap::PeekMut, net::SocketAddr};

use axum::{
    routing::{get, put},
    serve, Router,
};
use tokio::net::TcpListener;

pub async fn SimpleProgam() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/home", get(|| async { "home tab" }));

    let addr = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running simple program on http://localhost:3000");

    axum::serve(addr, app).await.unwrap();
}
