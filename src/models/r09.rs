pub mod dvb_dump {
  tonic::include_proto!("dvbdump");
}

pub use dvb_dump::R09GrpcTelegram;

use super::super::schema::r09_telegrams;
use super::{AuthenticationMeta, TelegramMetaInformation};

use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::time::SystemTime;
use std::hash::Hash;
use std::hash::Hasher;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct R09Telegram {
    pub telegram_type: u8,
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

#[derive(Deserialize, Serialize, Queryable, Insertable, Clone, PartialEq)]
#[table_name="r09_telegrams"]
pub struct R09SaveTelegram {
    pub id: i64,

    pub time: SystemTime,
    pub station: Uuid,
    pub region: i64, // foreign key references regions

    pub telegram_type: i16,
    pub delay: Option<i32>,
    pub reporting_point: i32,
    pub junction: i32, //derived from  reporting_point
    pub direction: i16, //derived from reporting_point
    pub request_status: i16, //derived from reporting_point
    pub priority: Option<i16>,
    pub direction_request: Option<i16>,
    pub line: Option<i32>,
    pub run_number: Option<i32>,
    pub destination_number: Option<i32>,
    pub train_length: Option<i16>,
    pub vehicle_number: Option<i32>,
    pub operator: Option<i16>,
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

            time: meta.time,
            station: meta.station,
            region: meta.region as i64,


            telegram_type: telegram.telegram_type as i16,
            delay: telegram.delay,
            reporting_point: telegram.reporting_point as i32,
            junction: telegram.junction as i32,
            direction: telegram.direction as i16,
            request_status: telegram.request_status as i16,
            priority: telegram.priority.map(|x| x as i16),
            direction_request: telegram.direction_request.map(|x| x as i16),
            line: telegram.line.map(|x| x as i32),
            run_number: telegram.run_number.map(|x| x as i32),
            destination_number: telegram.destination_number.map(|x| x as i32),
            train_length: telegram.train_length.map(|x| x as i16),
            vehicle_number: telegram.vehicle_number.map(|x| x as i32),
            operator: telegram.operator.map(|x| x as i16),
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
        self.telegram_type.hash(state);
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

impl fmt::Display for R09Telegram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type {} Line {:#?} Run {:#?} Destination {:#?} - {}",
            self.telegram_type,
            self.line,
            self.run_number,
            self.destination_number,
            self.request_status
        )
    }
}

impl R09GrpcTelegram {
    pub fn from(telegram: R09Telegram, meta: TelegramMetaInformation) -> R09GrpcTelegram {
        R09GrpcTelegram {
            time: meta.time.elapsed().unwrap().as_secs(),
            station: meta.station.to_string(),
            region: meta.region,

            telegram_type: telegram.telegram_type as u32,
            delay: telegram.delay,
            reporting_point: telegram.reporting_point,
            junction: telegram.junction as u32,
            direction: telegram.direction as u32,
            request_status: telegram.request_status as u32,
            priority: telegram.priority.map(|x| x as u32),
            direction_request: telegram.direction_request.map(|x| x as u32),
            line: telegram.line.map(|x| x as u32),
            run_number: telegram.run_number.map(|x| x as u32),
            destination_number: telegram.destination_number.map(|x| x as u32),
            train_length: telegram.train_length.map(|x| x as u32),
            vehicle_number: telegram.vehicle_number.map(|x| x as u32),
            operator: telegram.operator.map(|x| x as u32),
        }
    }
}

