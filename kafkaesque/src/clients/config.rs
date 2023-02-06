use derive_builder::Builder;
use derive_more::{Display, From, Into};
use itertools::Itertools;
use tokio::net::ToSocketAddrs;

#[derive(Debug, Builder, Clone)]
#[builder(pattern = "owned")]
pub struct ClientConfig {
    pub broker_list: BrokerList,
    pub client_id: ClientId,
}

#[derive(Debug, Default, From, Into, Clone)]
pub struct BrokerList(pub Vec<BrokerAddress>);

impl BrokerList {
    pub fn iter(&self) -> impl Iterator<Item = &BrokerAddress> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, From, Into, Display)]
pub struct ClientId(String);

impl<'a> From<&'a str> for ClientId {
    fn from(value: &'a str) -> Self {
        ClientId(value.into())
    }
}

#[derive(Debug, Clone, From, Into, Display)]
pub struct BrokerAddress(String);

impl From<&str> for BrokerAddress {
    fn from(value: &str) -> Self {
        BrokerAddress(value.to_string())
    }
}

impl BrokerAddress {
    pub fn as_to_socket_address(&self) -> impl ToSocketAddrs + '_ {
        self.0.as_str()
    }
}

impl BrokerList {
    pub fn from_csv(csv_list: impl AsRef<str>) -> Self {
        BrokerList(csv_list.as_ref().split(',').map_into().collect())
    }
}
