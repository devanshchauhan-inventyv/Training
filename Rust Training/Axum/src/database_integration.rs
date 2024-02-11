use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use serde_json::json;
use sqlx::{MySql, MySqlPool, Row};
use tokio::net::TcpListener;
async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT * from users").fetch_all(&pool).await {
        Ok(val) => val,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let user: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id":row.try_get::<i32,_>("id").unwrap_or_default(),
                "name":row.try_get::<String,_>("name").unwrap_or_default(),
                "email":row.try_get::<String,_>("email").unwrap_or_default()
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(user)).into_response()
}

pub async fn database_connect() {
    let database_url = "mysql://root:root@127.0.0.1/axum";
    let pool = match MySqlPool::connect(&database_url).await {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let app: Router = Router::new().route("/database", get(get_users).layer(Extension(pool)));

    let addr = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(addr, app).await.unwrap();
}
