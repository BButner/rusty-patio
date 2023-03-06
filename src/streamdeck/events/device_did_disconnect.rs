use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceDidDisconnectEvent {
    pub device: String,
    pub event: String,
}
