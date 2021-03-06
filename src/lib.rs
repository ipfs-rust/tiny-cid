//! # cid
//!
//! Implementation of [cid](https://github.com/ipld/cid) in Rust.
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

mod cid;
mod codec;
mod error;
mod version;

#[cfg(any(test, feature = "test"))]
mod arb;

pub use self::cid::Cid;
pub use self::codec::*;
pub use self::error::{Error, Result};
pub use self::version::Version;
