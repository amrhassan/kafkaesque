#![deny(warnings)]
#![deny(clippy::all)]

mod errors;
mod io;
mod types;

pub use errors::{KafkaesqueError, Result};
