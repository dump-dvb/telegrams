#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

pub use models::{
    r09::{
        R09Telegram,
        R09SaveTelegram,
        R09ReceiveTelegram,
        R09GrpcTelegram,
    },
    TelegramMetaInformation,
    AuthenticationMeta,
    ReceivesTelegramsClient,
    ReturnCode
};

