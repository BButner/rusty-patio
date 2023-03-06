use serde::{Deserialize, Serialize};

use crate::streamdeck::generic::device_info::DeviceInfo;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDidConnectEvent {
    pub event: String,
    pub device: String,
    pub device_info: DeviceInfo,
}
