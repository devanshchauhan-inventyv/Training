use std::{error::Error, fs, future::IntoFuture};

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put, put_service},
    Json, Router,
};
use serde::{de::value, Deserialize, Serialize};

use serde_json::json;
use tokio::net::TcpListener;

async fn read_data() -> Vec<Employee> {
    match fs::read_to_string(format!("./src/data/Master_Data.json")) {
        Ok(val) => {
            let data: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&val);
            match data {
                Ok(final_data) => final_data,
                Err(err) => {
                    println!("{:?}", err);
                    todo!("Serialization Error")
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            todo!("Input Output Error")
        }
    }
}

async fn show_users() -> impl IntoResponse {
    let users = read_data().await;
    Json(users)
}

async fn write_data(data: &Vec<Employee>) {
    let data = match serde_json::to_string_pretty(&data) {
        Ok(data) => data,
        Err(e) => {
            println!("{e}");
            todo!()
        }
    };
    match fs::write("./src/data/Master_Data.json", data) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    };
}

async fn delete_user(Path(id): Path<i64>) -> String {
    let delete_id = id;
    let mut data = read_data().await;
    data.retain(|emp| emp.id != id);
    write_data(&data).await;
    format!("the data with id {} is removed", delete_id)
}

async fn add_users(Json(data): Json<Employee>) -> impl IntoResponse {
    let mut vector = read_data().await;
    vector.push(data);
    write_data(&vector).await;
    Json(vector)
}

async fn update_user(Path(id): Path<i64>, Json(payload): Json<Employee>) -> Response {
    let search_id = id;
    let mut update = false;
    let mut data = read_data().await;
    for emp in data.iter_mut() {
        if emp.id == search_id {
            emp.name = payload.name.clone();
            emp.skills = payload.skills.clone();
            emp.status = payload.status.clone();
            emp.language = payload.language.clone();
            update = true;
        }
    }
    write_data(&data).await;
    if update {
        Json(payload).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

fn all_routes() -> Router {
    Router::new()
        .route("/crud/read", get(show_users))
        .route("/crud/create", post(add_users))
        .route("/crud/update/:id", put(update_user))
        .route("/crud/delete/:id", delete(delete_user))
}

pub async fn start_crud_server() {
    let app: Router = Router::new().merge(all_routes());

    let addr = TcpListener::bind("0.0.0.0:3002").await.unwrap();

    axum::serve(addr, app).await.unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub skills: Vec<String>,
    pub status: String,
    pub language: String,
}
