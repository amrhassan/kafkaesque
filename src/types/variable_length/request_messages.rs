pub enum ApiKey {
    ApiVersions = 18,
}

pub trait RequestMessage {
    const API_KEY: ApiKey;
}

pub struct ApiVersions;

impl RequestMessage for ApiVersions {
    const API_KEY: ApiKey = ApiKey::ApiVersions;
}
