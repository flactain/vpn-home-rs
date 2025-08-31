use ::serde::Serialize;
use serde::Deserialize;
use vpn_libs::entities::{clients::Client, terminals::Terminal};

//Client 作成Dto
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCreateDto {
    pub client_info: Client,
    pub terminal_info: Option<Terminal>,
}
