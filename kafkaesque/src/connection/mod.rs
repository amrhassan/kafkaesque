mod broker_connection;
mod errors;
mod lazy_broker_connection;

pub use broker_connection::BrokerConnection;
pub use errors::{ConnectionError, Result};
pub use lazy_broker_connection::LazyBrokerConnection;

/// Default TCP buffer size for [LazyBrokerConnection] in bytes
pub static DEFAULT_BUF_SIZE: usize = 8 * 1024;
