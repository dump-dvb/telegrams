pub mod dvb_dump {
  tonic::include_proto!("dvbdump");
}

use super::schema::r09_telegrams;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::SystemTime;
use std::hash::Hash;
use std::hash::Hasher;

use diesel::{Queryable, Insertable};

pub use dvb_dump::receives_telegrams_client::ReceivesTelegramsClient;
pub use dvb_dump::R09GrpcTelegram;

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

#[derive(Deserialize, Serialize, Debug, Queryable, Clone, Insertable)]
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
                       //                                           
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
            telegram_type: meta.telegram_type as i16,

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

impl R09GrpcTelegram {
    pub fn from(telegram: R09ReceiveTelegram, meta: TelegramMetaInformation) -> R09GrpcTelegram {
        R09GrpcTelegram {
            time: meta.time.elapsed().unwrap().as_secs(),
            station: meta.station.to_string(),
            region: meta.region,
            telegram_type: meta.telegram_type as u32,

            delay: telegram.data.delay,
            reporting_point: telegram.data.reporting_point,
            junction: telegram.data.junction as u32,
            direction: telegram.data.direction as u32,
            request_status: telegram.data.request_status as u32,
            priority: telegram.data.priority.map(|x| x as u32),
            direction_request: telegram.data.direction_request.map(|x| x as u32),
            line: telegram.data.line.map(|x| x as u32),
            run_number: telegram.data.run_number.map(|x| x as u32),
            destination_number: telegram.data.destination_number.map(|x| x as u32),
            train_length: telegram.data.train_length.map(|x| x as u32),
            vehicle_number: telegram.data.vehicle_number.map(|x| x as u32),
            operator: telegram.data.operator.map(|x| x as u32),
        }
    }
}

