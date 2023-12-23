use std::time::Instant;

use serde::{Serialize, Deserialize};
use num_bigint::{ToBigUint, BigUint};
use axum::{
    Router, Json,
    routing::get, http::StatusCode
};

#[derive(Serialize, Deserialize, Clone)]
struct FastFibRequest {
    n: i128
}

#[derive(Serialize, Deserialize, Clone)]
struct FastFibResponse {
    value: String,
    time: u128,
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/fib", get(get_fast_fib));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8181").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_fast_fib(Json(payload): Json<FastFibRequest>) -> (StatusCode, Json<FastFibResponse>){
    let start = Instant::now();
    let value = format!("{}", fast_fib(payload.n).await);
    let end = start.elapsed();
    println!("end time: {:?}", end);
    let r = FastFibResponse {
        value,
        time: end.as_millis(),
    };
    (StatusCode::OK, Json(r))
}


async fn fast_fib(target: i128) -> BigUint {
    if target <= 1 {
        return target.to_biguint().unwrap();
    }
    let mut dp: [BigUint; 2] = [0.to_biguint().unwrap(), 1.to_biguint().unwrap()];
    let mut n = 2;
    while n < target {
        dp.swap(0, 1);
        dp[1] = &dp[0] + &dp[1];
        n += 1;
    }
    &dp[0] + &dp[1]
}
