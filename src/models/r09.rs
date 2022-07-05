pub mod dvb_dump {
  tonic::include_proto!("dvbdump");
}

pub use dvb_dump::{R09GrpcTelegram};
use stop_names::Stop;

use super::super::schema::r09_telegrams;
use super::{AuthenticationMeta, TelegramMetaInformation};

use diesel::{Queryable, Insertable};
use serde::ser::{SerializeStruct, Serializer};
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
    pub region: String, // foreign key references regions

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

#[derive(Debug, Serialize)]
pub struct WebSocketTelegram {
    #[serde(flatten)]
    pub reduced: R09GrpcTelegram,

    #[serde(flatten)]
    pub meta_data: Stop,
}

impl R09SaveTelegram {
    pub fn from(telegram: R09Telegram, meta: TelegramMetaInformation) -> R09SaveTelegram {
        R09SaveTelegram {
            id: 0,

            time: meta.time,
            station: meta.station,
            region: meta.region,


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


/*
 *  uint64 time = 1;
    string station = 2;
    string region = 3;
    uint32 telegram_type = 4;
    optional int32 delay = 5;
    uint32 reporting_point = 6;
    uint32 junction = 7;
    uint32 direction = 8;
    uint32 request_status = 9;
    optional uint32 priority = 10;
    optional uint32 direction_request = 11;
    optional uint32 line = 12;
    optional uint32 run_number = 13;
    optional uint32 destination_number = 14;
    optional uint32 train_length = 15;
    optional uint32 vehicle_number = 16;
    optional uint32 operator = 17;
*/

impl Serialize for R09GrpcTelegram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("R09GrpcTelegram", 17)?;

        s.serialize_field("time", &self.time)?;
        s.serialize_field("station", &self.station)?;
        s.serialize_field("region", &self.region)?;
        s.serialize_field("telegram_type", &self.telegram_type)?;
        
        self.delay.map(|value| {
            s.serialize_field("delay", &value).ok();
        });

        s.serialize_field("reporting_point", &self.reporting_point)?;
        s.serialize_field("junction", &self.junction)?;
        s.serialize_field("direction", &self.direction)?;
        s.serialize_field("request_status", &self.request_status)?;

        self.priority.map(|value| {
            s.serialize_field("priority", &value).ok();
        });
        self.direction_request.map(|value| {
            s.serialize_field("direction_request", &value).ok();
        });
        self.line.map(|value| {
            s.serialize_field("line", &value).ok();
        });
        self.run_number.map(|value| {
            s.serialize_field("run_number", &value).ok();
        });
        self.destination_number.map(|value| {
            s.serialize_field("destination_number", &value).ok();
        });
        self.train_length.map(|value| {
            s.serialize_field("train_length", &value).ok();
        });
        self.vehicle_number.map(|value| {
            s.serialize_field("vehicle_number", &value).ok();
        });
        self.operator.map(|value| {
            s.serialize_field("operator", &value).ok();
        });

        s.end()
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

