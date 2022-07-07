pub mod r09;

pub mod dvb_dump {
    tonic::include_proto!("dvbdump");
}

pub use dvb_dump::receives_telegrams_client::ReceivesTelegramsClient;
pub use dvb_dump::receives_telegrams_server::ReceivesTelegrams;
pub use dvb_dump::receives_telegrams_server::ReceivesTelegramsServer;
pub use dvb_dump::{R09GrpcTelegram, ReturnCode};

use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

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
