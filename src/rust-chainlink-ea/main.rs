// RUST_LOG='rust-cl-ea=info' ./target/release/rust-chainlink-ea

#[macro_use]
extern crate lazy_static;

mod conflux;
mod model;

use cfx_transaction::{privkey_to_address, Action, Transaction};
use ethereum_types::{H160, H256};
use log::{debug, info};
use warp::Filter;

use conflux::{get_account_nonce, get_latest_epoch, send_transaction};
use model::{Job, JobResult};

const ADDRESS: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 1234;
const CONFLUX_NODE_URL: &str = "http://test.confluxrpc.org";

lazy_static! {
    pub static ref PRIVATE_KEY: H256 ="[ENTER PRIVATE KEY]".parse().unwrap();
    pub static ref WEB3: web3::Web3<web3::transports::Http> =
        web3::Web3::new(web3::transports::Http::new(CONFLUX_NODE_URL).unwrap());
}

#[derive(Debug)]
struct Error(String);
impl warp::reject::Reject for Error {}

fn reject(e: impl std::error::Error) -> warp::Rejection {
    warp::reject::custom(Error(e.to_string()))
}

fn reject_str(e: String) -> warp::Rejection {
    warp::reject::custom(Error(e))
}

pub fn parse_job() -> impl Filter<Extract = (Job,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn process_job(job: Job) -> Result<impl warp::Reply, warp::Rejection> {
    info!(target: "rust-cl-ea", "new job request: {:?}", job);

    let privkey: H256 = *PRIVATE_KEY;

    // parse input
    let oracle = job
        .data
        .address
        .trim_start_matches("0x")
        .parse::<H160>()
        .map_err(reject)?;

    let function_selector = job.data.function_selector.trim_start_matches("0x");
    let data_prefix = job.data.data_prefix.trim_start_matches("0x");

    let result_int = u64::from_str_radix(&job.data.result, 10).map_err(reject)?;
    let result_padded = format!("{:0>64x}", result_int);

    let data = format!("{}{}{}", function_selector, data_prefix, result_padded);
    let data = hex::decode(data).map_err(reject)?;

    // assemble transaction
    let sender = privkey_to_address(privkey).expect("Incorrect private key");
    let sender = format!("{:?}", sender);

    let epoch_height = get_latest_epoch(&WEB3).await.map_err(reject_str)?;
    let nonce = get_account_nonce(&WEB3, sender).await.map_err(reject_str)?;

    let tx = Transaction {
        action: Action::Call(oracle),
        chain_id: 1,
        data,
        epoch_height,
        gas: 500_000.into(),
        gas_price: 1.into(),
        nonce,
        storage_limit: 1000,
        value: 0.into(),
    };

    let signed = tx.sign(&privkey.into());
    debug!(target: "rust-cl-ea", "signed transaction: {:?}", signed);

    let tx_hash = send_transaction(&WEB3, &signed).await.unwrap();
    info!(target: "rust-cl-ea", "oracle fulfillment transaction sent: {}", tx_hash);

    Ok(warp::reply::json(&JobResult { id: job.id }))
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
