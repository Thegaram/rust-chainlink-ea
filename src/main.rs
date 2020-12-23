// RUST_LOG='rust-cl-ea=info' ./target/release/rust-chainlink-ea

use log::info;
use std::convert::Infallible;
use warp::Filter;

const ADDRESS: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 1234;

async fn process_job(job: serde_json::Value) -> Result<impl warp::Reply, Infallible> {
    info!(target: "rust-cl-ea", "new job request: {:?}", job);
    Ok(warp::http::StatusCode::OK)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /health
    let health = warp::path!("health")
        .and(warp::get())
        .map(|| "OK");

    // POST /
    let job = warp::path::end()
        .and(warp::post())
        .and(warp::body::json())
        .and_then(process_job);

    let routes = health.or(job).with(warp::log("rust-cl-ea"));

    info!(target: "rust-cl-ea", "Starting server at {:?}:{:?}", ADDRESS, PORT);
    warp::serve(routes).run((ADDRESS, PORT)).await;
}
