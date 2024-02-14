pub mod health_check;
pub mod model;
pub mod routes;
use axum::Router;
use health_check::*;
use model::*;
use routes::*;
mod middlewares;
use middlewares::*;
use tokio::net::TcpListener;

pub async fn start_axum_task_server() {
    let app = get_routes();
    let app=get_middlewares(app);
    // let app: Router = Router::new().merge(app);

    let addr = TcpListener::bind("0.0.0.0:3001").await.unwrap();
        load_data_to_queues().await;
    axum::serve(addr, app).await.unwrap();
}
