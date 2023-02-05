#![feature(async_fn_in_trait)]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(unused)]
#![allow(incomplete_features)]

mod errors;
pub mod protocol;

pub use errors::Result;
pub use protocol::Client;
