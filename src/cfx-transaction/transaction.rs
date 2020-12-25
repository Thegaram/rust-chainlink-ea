// Copyright 2019 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use super::{recover, sign, Action, Error, Public, Secret, Signature};
use ethcore_bytes::Bytes;
use ethereum_types::{H256, U256};
use keccak_hash::keccak;
use rlp::{self, RlpStream};
use rlp_derive::{RlpDecodable, RlpEncodable};

#[derive(Default, Debug, Clone, Eq, PartialEq, RlpEncodable, RlpDecodable)]
pub struct Transaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub action: Action,
    pub value: U256,
    pub storage_limit: u64,
    pub epoch_height: u64,
    pub chain_id: u32,
    pub data: Bytes,
}

#[derive(Debug, Clone, Eq, PartialEq, RlpEncodable, RlpDecodable)]
pub struct TransactionWithSignature {
    pub unsigned: Transaction,
    pub v: u8,
    pub r: U256,
    pub s: U256,
}

impl Transaction {
    pub fn hash(&self) -> H256 {
        let mut s = RlpStream::new();
        s.append(self);
        keccak(s.as_raw())
    }

    pub fn sign(self, secret: &Secret) -> TransactionWithSignature {
        let sig = sign(secret, &self.hash())
            .expect("data is valid and context has signing capabilities; qed");

        TransactionWithSignature {
            unsigned: self,
            r: sig.r().into(),
            s: sig.s().into(),
            v: sig.v(),
        }
    }
}

impl TransactionWithSignature {
    pub fn signature(&self) -> Signature {
        let mut r = H256::zero();
        self.r.to_big_endian(r.as_bytes_mut());

        let mut s = H256::zero();
        self.s.to_big_endian(s.as_bytes_mut());

        Signature::from_rsv(&r, &s, self.v)
    }

    pub fn recover_public(&self) -> Result<Public, Error> {
        Ok(recover(&self.signature(), &self.unsigned.hash())?)
    }

    pub fn into_hex_string(&self) -> String {
        let mut s = RlpStream::new();
        s.append(self);
        let x = s.as_raw();
        hex::encode(x)
    }
}
