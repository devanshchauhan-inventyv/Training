use axum::{middleware, routing::{get, post}, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use super::{
     cors::create_cors_layer, create_user_cust, create_user_emp, create_user_stud, delete_user, get_middlewares, health_check::health_route, read_all_users, read_user, update_user_cust, update_user_emp, update_user_stud
};

pub fn get_routes() -> Router {
    let app = Router::new().merge(health_route()).merge(get_user_routes());
    app
}

fn get_user_routes() -> Router {
    Router::new()
        .route("/employee/create", post(create_user_emp))
        .route("/student/create", post(create_user_stud))
        .route("/customer_service/create", post(create_user_cust))
        .route("/:task/read_all", post(read_all_users))
        .route("/employee/update/:id", post(update_user_emp))
        .route("/student/update/:id", post(update_user_stud))
        .route("/customer_service/update/:id", post(update_user_cust))
        .route("/:task/delete/:id", post(delete_user))
        .route("/:task/read/:id", post(read_user))
        
}
