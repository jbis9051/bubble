use bridge_macro::bridge;
use common::base64::Base64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    Location(Location),
    GroupStatus(GroupStatus),
}

#[derive(Debug, Deserialize, Serialize)]
#[bridge]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupStatus {
    pub name: Option<String>,
    pub image: Option<Base64>,
}
