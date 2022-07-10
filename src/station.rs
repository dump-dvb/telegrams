use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RadioStation {
    pub id: Uuid,
    pub name: String,
    pub region: u32,
    pub lat: f64,
    pub lon: f64
}

