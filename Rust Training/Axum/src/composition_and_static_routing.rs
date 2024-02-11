use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn compostion_and_static_routing() {
    let routes_all: Router = Router::new().merge(all_routes());
    // .fallback_service(routes_static());
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let addr = TcpListener::bind("0.0.0.1:3001").await.unwrap();
    axum::serve(addr, routes_all).await.unwrap();
}

// Composition of routes
fn all_routes() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/:name", get(handler2))
}

// Static routes
// fn routes_static() -> Router {
//     Router::new()
//         .nest_service("/", get_service(get_service(ServeDir::new("./"))))
// }

#[derive(Debug, Deserialize)]
struct NameParameter {
    name: Option<String>,
}

async fn handler(Query(params): Query<NameParameter>) -> impl IntoResponse {
    println!("->> {:<12} -handler-{params:?}", "HANDLER");
    let name = params.name.unwrap_or("Coder".to_string());

    Html(format!("Hello {}", name))
}

async fn handler2(Path(name): Path<NameParameter>) -> impl IntoResponse {
    println!("->> {:<12} -handler-{name:?}", "HANDLER");
    let name = name.name.unwrap_or("Coder".to_string());

    Html(format!("Hello {}", name))
}
