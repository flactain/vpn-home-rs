use std::str::FromStr;

use defguard_wireguard_rs::{InterfaceConfiguration, host::Peer, key::Key, net::IpAddrMask};
use ipnet::IpNet;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Clone, Deserialize, Debug)]
pub struct HostConfig {
    pub config_name: String,
    #[sqlx(skip)]
    pub private_key: Option<String>,
    pub address: IpNet,
    pub port: i32,
}

impl From<HostConfig> for InterfaceConfiguration {
    fn from(value: HostConfig) -> Self {
        let mut addresses = Vec::new();
        let address: IpAddrMask = IpAddrMask::from_str(value.address.to_string().as_str()).unwrap();
        addresses.push(address);

        InterfaceConfiguration {
            name: value.config_name,
            prvkey: value.private_key.unwrap(),
            addresses,
            port: (value.port.unsigned_abs()),
            peers: vec![],
            mtu: None,
        }
    }
}

pub struct PeerConfig {
    pub public_key: String,
    pub allowed_ip: IpAddrMask,
    pub persistent_keepalive_interval: Option<u16>,
}

impl From<&PeerConfig> for Peer {
    fn from(value: &PeerConfig) -> Self {
        let mut allowed_ips = Vec::new();
        allowed_ips.push(value.allowed_ip.clone());
        //kari
        let mut peer = Peer::new(Key::generate());
        peer.allowed_ips = allowed_ips;
        peer.persistent_keepalive_interval = value.persistent_keepalive_interval;

        peer
    }
}
