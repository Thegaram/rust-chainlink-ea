// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use super::Error;
use ethereum_types::{H160, H256, H512};
use parity_crypto::Keccak256 as _;
use secp256k1::{key::SecretKey, Message as SecpMessage, RecoverableSignature, RecoveryId};
use std::{ops::Deref, str::FromStr};
use zeroize::Zeroize;

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

#[derive(Clone, PartialEq, Eq)]
pub struct Secret {
    inner: H256,
}

impl Drop for Secret {
    fn drop(&mut self) {
        self.inner.0.zeroize()
    }
}

impl Deref for Secret {
    type Target = H256;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromStr for Secret {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(H256::from_str(s)
            .map_err(|e| Error::Custom(format!("{:?}", e)))?
            .into())
    }
}

impl From<[u8; 32]> for Secret {
    fn from(k: [u8; 32]) -> Self {
        Secret { inner: H256(k) }
    }
}

impl From<H256> for Secret {
    fn from(s: H256) -> Self {
        s.0.into()
    }
}

impl From<&'static str> for Secret {
    fn from(s: &'static str) -> Self {
        s.parse()
            .unwrap_or_else(|_| panic!("invalid string literal for {}: '{}'", stringify!(Self), s))
    }
}

pub type Message = H256;

pub type Public = H512;
#[repr(C)]
pub struct Signature([u8; 65]);

impl Signature {
    pub fn r(&self) -> &[u8] {
        &self.0[0..32]
    }
    pub fn s(&self) -> &[u8] {
        &self.0[32..64]
    }
    pub fn v(&self) -> u8 {
        self.0[64]
    }

    /// Create a signature object from the sig.
    pub fn from_rsv(r: &H256, s: &H256, v: u8) -> Self {
        let mut sig = [0u8; 65];
        sig[0..32].copy_from_slice(r.as_ref());
        sig[32..64].copy_from_slice(s.as_ref());
        sig[64] = v;
        Signature(sig)
    }
}

impl Deref for Signature {
    type Target = [u8; 65];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn sign(secret: &Secret, message: &Message) -> Result<Signature, Error> {
    let context = &SECP256K1;
    let sec = SecretKey::from_slice(context, secret.as_ref())?;
    let s = context.sign_recoverable(&SecpMessage::from_slice(&message[..])?, &sec)?;
    let (rec_id, data) = s.serialize_compact(context);
    let mut data_arr = [0; 65];

    // // no need to check if s is low, it always is
    data_arr[0..64].copy_from_slice(&data[0..64]);
    data_arr[64] = rec_id.to_i32() as u8;
    Ok(Signature(data_arr))
}

pub fn recover(signature: &Signature, message: &Message) -> Result<Public, Error> {
    let context = &SECP256K1;
    let rsig = RecoverableSignature::from_compact(
        context,
        &signature[0..64],
        RecoveryId::from_i32(signature[64] as i32)?,
    )?;
    let pubkey = context.recover(&SecpMessage::from_slice(&message[..])?, &rsig)?;
    let serialized = pubkey.serialize_vec(context, false);

    let mut public = Public::default();
    public.as_bytes_mut().copy_from_slice(&serialized[1..65]);
    Ok(public)
}

pub fn public_to_address(public: &Public) -> H160 {
    let hash = public.keccak256();
    let mut result = H160::zero();
    result.as_bytes_mut().copy_from_slice(&hash[12..]);
    // In Conflux, we reserve the first four bits to indicate the type of the
    // address. For user address, the first four bits will be 0x1.
    let type_byte = &mut result.as_fixed_bytes_mut()[0];
    *type_byte &= 0x0f;
    *type_byte |= 0x10;
    result
}
