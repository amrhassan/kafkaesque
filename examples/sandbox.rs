use kafkaesque::protocol::messages::{ApiVersionsRequest, ApiVersionsResponse};
use kafkaesque::{Client, Result};
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();
    let mut client = Client::connect("amr-plays", "localhost:9092").await?;
    let resp: ApiVersionsResponse = client.send(ApiVersionsRequest).await?.message;
    info!("Got response: {resp:?}");
    Ok(())
}

fn setup_tracing() {
    let tracing_fmt_layer = fmt::layer().with_target(true).with_ansi(true);
    let tracing_filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug,tokio=trace"))
        .expect("Failed to set global logging filter");
    tracing_subscriber::registry()
        .with(tracing_filter_layer)
        .with(tracing_fmt_layer)
        .init();
    info!("Initiated logging");
}
