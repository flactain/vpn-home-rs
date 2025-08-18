use defguard_wireguard_rs::{InterfaceConfiguration, Userspace, WGApi, WireguardInterfaceApi};

use crate::entities::wireguard::{HostConfig, PeerConfig};

pub async fn reconstruct_interface(
    host_config: HostConfig,
    peer_configs: Vec<PeerConfig>,
) -> anyhow::Result<()> {
    let wgapi = WGApi::<Userspace>::new(host_config.config_name.clone())?;
    let interface_configuration: InterfaceConfiguration = host_config.into();

    //既存のモノなら再設定、新規なら作成
    match wgapi.read_interface_data() {
        Ok(_) => wgapi.configure_interface(&interface_configuration)?,
        Err(_) => wgapi.configure_interface(&interface_configuration)?,
    };

    //クライアントの追加
    peer_configs
        .iter()
        .for_each(|peer| wgapi.configure_peer(&peer.into()).unwrap());

    //ここまで来れば成功！
    Ok(())
}
