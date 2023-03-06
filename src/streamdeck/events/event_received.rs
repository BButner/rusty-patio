use serde::{Deserialize, Serialize};

use super::{
    device_did_connect::DeviceDidConnectEvent, device_did_disconnect::DeviceDidDisconnectEvent,
    event_title::StreamDeckEventTitle,
};

pub enum EventReceived {
    DeviceDidConnect(DeviceDidConnectEvent),
    DeviceDidDisconnect(DeviceDidDisconnectEvent),
    UnknownEvent(String),
}

impl EventReceived {
    pub fn from_json(json: &str) -> Result<EventReceived, Box<dyn std::error::Error>> {
        let event_base: EventBase = serde_json::from_str(json)?;

        match event_base.event.as_str() {
            StreamDeckEventTitle::DEVICE_DID_CONNECT => {
                Ok(EventReceived::DeviceDidConnect(serde_json::from_str(json)?))
            }
            StreamDeckEventTitle::DEVICE_DID_DISCONNECT => Ok(EventReceived::DeviceDidDisconnect(
                serde_json::from_str(json)?,
            )),
            _ => Ok(EventReceived::UnknownEvent(event_base.event)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventBase {
    pub event: String,
}
