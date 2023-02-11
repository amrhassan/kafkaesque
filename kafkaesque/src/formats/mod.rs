mod api_keys;
mod broker_connection;
mod codec;
mod error_code;
mod errors;
mod fixed_lengths;
mod request;
mod variable_lengths;

pub mod messages;

pub use api_keys::ApiKey;
pub use broker_connection::BrokerConnection;
pub use codec::{FixedLength, Read, Write};
pub use error_code::ErrorCode;
pub use errors::{FormatError, Result};
pub use request::{ApiVersion, RequestMessage};
pub use variable_lengths::NullableString;

/// Default TCP buffer size for [BrokerConnection] in bytes
pub static DEFAULT_BUF_SIZE: usize = 8 * 1024;
