use std::{
    future::{Future, IntoFuture},
    time::Duration,
};

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    function_display().await;
}

async fn function_display() {
    for i in 0..2 {
        let res = data().await;
        println!("[{}] {}", i,res);
        let res2 = data().await;
        println!("[{}] {}",i, res2);
    }
}

async fn data() -> String {
    sleep(Duration::from_secs(2)).await;
    "hello".to_owned()
}
