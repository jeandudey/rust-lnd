#[cfg(feature = "bitcoind")]
extern crate bitcoin_rpc;
extern crate bitcoin_amount;

extern crate lightning;

#[cfg(feature = "bitcoind")]
pub mod bitcoind;
