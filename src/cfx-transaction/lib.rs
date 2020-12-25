// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Copyright 2019 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

#[macro_use]
extern crate lazy_static;

mod action;
mod error;
mod signature;
mod transaction;
mod util;

use error::Error;
use signature::{recover, sign, Public, Signature};

pub use action::Action;
pub use signature::{public_to_address, Secret};
pub use transaction::{Transaction, TransactionWithSignature};
pub use util::privkey_to_address;
