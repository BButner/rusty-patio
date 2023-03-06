use serde::{Deserialize, Serialize};

use crate::streamdeck::arguments::StreamDeckSize;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub size: StreamDeckSize,
    #[serde(rename = "type")]
    pub device_type: i32,
}
