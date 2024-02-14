use axum::{http::{HeaderMap, HeaderName, HeaderValue}, response::{IntoResponse, Response}, routing::get, Json, Router};
use tokio::net::TcpListener;

async fn get_user_agent(header:HeaderMap)-> String{
   let data= header.get("User-Agent").unwrap().clone();
   data.to_str().unwrap().to_owned()
}

pub async fn user_agent(){
    let app: Router=Router::new().route("/useragent", get(get_user_agent));

    let addr= TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(addr, app).await.unwrap();

}