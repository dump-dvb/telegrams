pub mod r09;

pub mod dvb_dump {
  tonic::include_proto!("dvbdump");
}

pub use dvb_dump::{ReturnCode};
pub use dvb_dump::receives_telegrams_client::ReceivesTelegramsClient;
pub use dvb_dump::receives_telegrams_server::ReceivesTelegrams;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::SystemTime;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TelegramMetaInformation {
    pub time: SystemTime,
    pub station: Uuid,
    pub region: u64, // foreign key references regions
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthenticationMeta {
    pub station: Uuid,
    pub token: String,
}
