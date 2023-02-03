#![feature(async_fn_in_trait)]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(unused)]
#![allow(incomplete_features)]

mod broker;
mod client;
mod errors;
mod io;
mod types;

pub use errors::Result;
