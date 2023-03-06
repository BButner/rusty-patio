use serde::{Deserialize, Serialize};

use crate::streamdeck::generic::device_info::DeviceInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceDidConnectEvent {
    pub event: String,
    pub device: String,
    #[serde(rename = "deviceInfo")]
    pub device_info: DeviceInfo,
}
