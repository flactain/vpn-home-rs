use defguard_wireguard_rs::{InterfaceConfiguration, Kernel, WGApi, WireguardInterfaceApi};
use vpn_libs::entities::errors::AppError;

use crate::entities::wireguard::{HostConfig, PeerConfig};

pub struct WireguardClient {
    wg_api: WGApi<Kernel>,
    host_configuration: InterfaceConfiguration,
}

impl WireguardClient {
    pub fn new(host_config: HostConfig) -> Result<Self, AppError> {
        let wg_api = WGApi::<Kernel>::new(host_config.config_name.clone());
        let interface_configuration = host_config.into();
        match wg_api {
            Ok(wg_api) => {
                wg_api.create_interface().unwrap();
                wg_api
                    .configure_interface(&interface_configuration)
                    .unwrap();
                Ok(Self {
                    wg_api,
                    host_configuration: interface_configuration,
                })
            }
            Err(err) => Err(AppError::VpnConfigurationError(err)),
        }
    }

    pub fn add_peers(&self, peer_configs: Vec<PeerConfig>) -> anyhow::Result<()> {
        //クライアントの追加
        peer_configs
            .iter()
            .for_each(|peer| self.wg_api.configure_peer(&peer.into()).unwrap());

        Ok(())
    }
}
