use ::serde::Serialize;
use serde::Deserialize;
use vpn_libs::entities::{clients::ClientOutline, terminals::TerminalOutline};

//Client 作成Dto
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCreateDto {
    pub client_info: ClientOutline,
    pub terminal_info: Option<TerminalOutline>,
}
