use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::SystemTime;
use std::hash::Hash;
use std::hash::Hasher;

use diesel::{Queryable, Insertable};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct R09Telegram {
    pub delay: Option<i32>,
    pub reporting_point: u32,
    pub junction: u32, //derived from  reporting_point
    pub direction: u8, //derived from reporting_point
    pub request_status: u8, //derived from reporting_point
    pub priority: Option<u8>,
    pub direction_request: Option<u8>,
    pub line: Option<u32>,
    pub run_number: Option<u32>,
    pub destination_number: Option<u32>,
    pub train_length: Option<u8>,
    pub vehicle_number: Option<u32>,
    pub operator: Option<u8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TelegramMetaInformation {
    pub time: SystemTime,
    pub station: Uuid,
    pub region: u64, // foreign key references regions
    pub telegram_type: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthenticationMeta {
    pub station: Uuid,
    pub token: String,
    pub telegram_type: u8,
}

#[derive(Deserialize, Serialize, Debug, Queryable, Clone)]
pub struct R09SaveTelegram {
    pub id: u64,

    #[serde(flatten)]
    pub meta_data: TelegramMetaInformation,

    #[serde(flatten)]
    pub data: R09Telegram,
}

#[derive(Deserialize, Serialize, Debug, Queryable, Clone)]
pub struct R09ReceiveTelegram {
    #[serde(flatten)]
    pub auth: AuthenticationMeta,

    #[serde(flatten)]
    pub data: R09Telegram,
}


impl R09SaveTelegram {
    pub fn from(telegram: R09Telegram, meta: TelegramMetaInformation) -> R09SaveTelegram {
        R09SaveTelegram {
            id: 0,
            data: telegram,
            meta_data: meta
        }
    }
}

impl Hash for R09ReceiveTelegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Hash for R09Telegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.delay.hash(state);
        self.reporting_point.hash(state);
        self.junction.hash(state);
        self.direction.hash(state);
        self.request_status.hash(state);
        self.priority.hash(state);
        self.direction_request.hash(state);
        self.line.hash(state);
        self.run_number.hash(state);
        self.destination_number.hash(state);
        self.train_length.hash(state);
        self.vehicle_number.hash(state);
        self.operator.hash(state);
    }
}

