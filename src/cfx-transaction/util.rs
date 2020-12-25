use super::{public_to_address, Transaction};
use ethereum_types::{H160, H256};

pub fn privkey_to_address(privkey: H256) -> Result<H160, String> {
    let tx = Transaction::default();
    let signed = tx.sign(&privkey.into());
    let public = signed.recover_public().map_err(|e| e.to_string())?;
    Ok(public_to_address(&public))
}
