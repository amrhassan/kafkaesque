//! Unofficial Kafka implementations

#![feature(async_fn_in_trait)]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(incomplete_features)]

/// Clients to Kafka brokers
pub mod clients;

/// Low-level formats and their codecs
pub mod formats;
