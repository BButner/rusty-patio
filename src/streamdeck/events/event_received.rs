use serde::{Deserialize, Serialize};

use super::device_did_connect::DeviceDidConnectEvent;

pub enum EventReceived {
    DeviceDidConnect(DeviceDidConnectEvent),
}

impl EventReceived {
    pub fn from_json(json: &str) -> Result<EventReceived, Box<dyn std::error::Error>> {
        let event_base: EventBase = serde_json::from_str(json).unwrap();

        match event_base.event.as_str() {
            "deviceDidConnect" => Ok(EventReceived::DeviceDidConnect(
                serde_json::from_str(json).unwrap(),
            )),
            // _ => panic!("Unknown event type: {}", event_base.event),
            _ => Err(format!("Unknown event type: {}", event_base.event).into()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventBase {
    pub event: String,
}
