//! Unofficial Kafka implementations

#![feature(lazy_cell)]
#![feature(async_fn_in_trait)]
#![deny(warnings)]
#![deny(clippy::all)]
#![allow(incomplete_features)]

/// Runtime configurations
pub mod config;

/// Two way communication with brokers
pub mod connection;

/// Common models
pub mod models;

/// Low-level clients to Kafka brokers
pub mod clients;

/// Low-level formats and their codecs
pub mod formats;

/// Message producer
pub mod producer;

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;
    use tracing::info;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    pub fn setup_tracing() {
        static ONCE: LazyLock<()> = LazyLock::new(|| {
            let tracing_fmt_layer = fmt::layer().with_target(false).with_ansi(true);
            let tracing_filter_layer = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("error"))
                .expect("Failed to set global logging filter");
            tracing_subscriber::registry()
                .with(tracing_filter_layer)
                .with(tracing_fmt_layer)
                .init();
            info!("Initiated tracing");
        });
        LazyLock::force(&ONCE);
    }
}
