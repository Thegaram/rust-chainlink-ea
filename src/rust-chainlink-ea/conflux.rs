use cfx_transaction::TransactionWithSignature;
use ethereum_types::U256;
use log::debug;
use web3::{Transport, Web3};

pub async fn get_account_nonce(
    web3: &Web3<impl Transport>,
    account: String,
) -> Result<U256, String> {
    debug!("cfx_getNextNonce({:?})", account);

    let raw = web3
        .transport()
        .execute("cfx_getNextNonce", vec![account.into()])
        .await
        .map_err(|e| format!("RPC error: {:?}", e))?;

    debug!("cfx_getNextNonce response: {:?}", raw);

    raw.as_str()
        .ok_or_else(|| "Unexpected RPC response")?
        .trim_start_matches("0x")
        .parse::<U256>()
        .map_err(|e| e.to_string())
}

pub async fn get_latest_epoch(web3: &Web3<impl Transport>) -> Result<u64, String> {
    debug!("cfx_epochNumber()");

    let raw = web3
        .transport()
        .execute("cfx_epochNumber", vec![])
        .await
        .map_err(|e| format!("RPC error: {:?}", e))?;

    debug!("cfx_epochNumber response: {:?}", raw);

    let raw = raw
        .as_str()
        .ok_or_else(|| "Unexpected RPC response")?
        .trim_start_matches("0x");

    u64::from_str_radix(raw, 16).map_err(|e| e.to_string())
}

pub async fn send_transaction(
    web3: &Web3<impl Transport>,
    tx: &TransactionWithSignature,
) -> Result<String, String> {
    let hex = format!("0x{}", tx.into_hex_string());

    debug!("cfx_sendRawTransaction({:?})", hex);

    let raw = web3
        .transport()
        .execute("cfx_sendRawTransaction", vec![hex.into()])
        .await
        .map_err(|e| format!("RPC error: {:?}", e))?;

    debug!("cfx_sendRawTransaction response: {:?}", raw);

    let hash = raw
        .as_str()
        .ok_or_else(|| "Unexpected RPC response")?
        .to_owned();

    Ok(hash)
}
