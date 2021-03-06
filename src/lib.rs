//! Common types and data for the ssb ecosystem
#![deny(missing_docs)]

extern crate sodiumoxide;
extern crate base64;
extern crate dirs;
extern crate regex;
extern crate serde;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;


/// The default tcp port of an ssb server.
pub const DEFAULT_TCP_PORT: u16 = 8008;
/// The default websocket port of an ssb server.
pub const DEFAULT_WS_PORT: u16 = 8989;
/// The default blobs port of of an ssb server.
pub const DEFAULT_BLOBS_PORT: u16 = 7777;

/// The network identifier of the main ssb network.
pub const MAINNET_IDENTIFIER: [u8; 32] =
    [212, 161, 203, 136, 166, 111, 2, 248, 219, 99, 92, 226, 100, 65, 204, 93, 172, 27, 8, 66, 12,
     234, 172, 35, 8, 57, 183, 85, 132, 90, 159, 251];

pub mod directory;
pub mod hashes;
pub mod keys;
pub mod links;
pub mod messages;
