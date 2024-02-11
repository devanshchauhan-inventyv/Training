use std::net::SocketAddr;

use axum::{
    extract::Query,
    response::Html,
    routing::{get, Route},
    Router,
};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct RangeParameter {
    start: usize,
    end: usize,
}

pub async fn genrate_random_number() {
    let app: Router = Router::new().route("/", get(handler));

    // let addr=SocketAddr::from(([127,0,0,1],3001));
    let addr = TcpListener::bind("0.0.0.1:3001").await.unwrap();

    println!("Running on generate random number on {:#?}", &addr);

    axum::serve(addr, app).await.unwrap();
}
async fn handler(Query(range): Query<RangeParameter>) -> Html<String> {
    let random_number = thread_rng().gen_range(range.start..=range.end);
    Html(format!("<h1>the random number is {}</h1>", random_number))
}
