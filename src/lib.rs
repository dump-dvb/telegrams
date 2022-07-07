#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

pub use models::{
    r09::{R09ReceiveTelegram, R09SaveTelegram, R09Telegram},
    AuthenticationMeta, R09GrpcTelegram, ReceivesTelegrams, ReceivesTelegramsClient,
    ReceivesTelegramsServer, ReturnCode, TelegramMetaInformation,
};
