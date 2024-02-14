use axum::Router;
use tower::ServiceBuilder;

use self::cors::create_cors_layer;

pub mod cors;
pub fn get_middlewares(route: Router) -> Router {
    route.layer(ServiceBuilder::new().layer(create_cors_layer()))
}