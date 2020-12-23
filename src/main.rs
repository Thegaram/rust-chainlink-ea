// RUST_LOG='rust-cl-ea=info' ./target/release/rust-chainlink-ea

use log::info;
use serde::{Serialize, Deserialize};
use std::convert::Infallible;
use warp::Filter;

const ADDRESS: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 1234;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Job {
    id: String,
    data: JobData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct JobData {
    address: String,
    data_prefix: String,
    function_selector: String,
    result: String,
}

fn parse_job() -> impl Filter<Extract = (Job,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn process_job(job: Job) -> Result<impl warp::Reply, Infallible> {
    info!(target: "rust-cl-ea", "new job request: {:?}", job);

    // Transaction {
    //     from: wallet,
    //     to: job.data.address,
    //     data: encode(job.data.function_selector, job.data.data_prefix, job.data.result),
    //     // ...
    // }

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
        .and(parse_job())
        .and_then(process_job);

    let routes = health.or(job).with(warp::log("rust-cl-ea"));

    info!(target: "rust-cl-ea", "Starting server at {:?}:{:?}", ADDRESS, PORT);
    warp::serve(routes).run((ADDRESS, PORT)).await;
}
