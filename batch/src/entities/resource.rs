use vpn_libs::entities::{clients::Client, vpns::VpnOutline};

pub enum ProcessResourceType {
    Client(Client),
    Vpn(VpnOutline),
}
