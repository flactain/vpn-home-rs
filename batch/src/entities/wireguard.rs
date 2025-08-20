use defguard_wireguard_rs::{InterfaceConfiguration, host::Peer, key::Key, net::IpAddrMask};

pub struct HostConverter;

impl HostConverter {
    pub fn new(host_config: HostConfig, peer_configs: Vec<PeerConfig>) -> InterfaceConfiguration {
        let mut addresses = Vec::new();
        addresses.push(host_config.address);

        InterfaceConfiguration {
            name: host_config.config_name,
            prvkey: host_config.private_key.unwrap(),
            addresses,
            port: (host_config.port),
            peers: peer_configs.iter().map(Peer::from).collect(),
            mtu: None,
        }
    }
}

pub struct HostConfig {
    pub config_name: String,
    pub private_key: Option<String>,
    pub address: IpAddrMask,
    pub port: u32,
}

impl From<HostConfig> for InterfaceConfiguration {
    fn from(value: HostConfig) -> Self {
        let mut addresses = Vec::new();
        addresses.push(value.address);
        InterfaceConfiguration {
            name: value.config_name,
            prvkey: value.private_key.unwrap(),
            addresses,
            port: (value.port),
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
