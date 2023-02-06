#![feature(async_fn_in_trait)]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(incomplete_features)]

mod errors;
pub mod protocol;

pub use errors::{KafkaesqueError, Result};
pub use protocol::{BrokerConnection, Response};
