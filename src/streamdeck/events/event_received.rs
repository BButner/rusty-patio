use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    application_monitor_event::ApplicationMonitorEvent, device_did_connect::DeviceDidConnectEvent,
    device_did_disconnect::DeviceDidDisconnectEvent, event_title::StreamDeckEventTitle,
    key_down::KeyDownEvent, system_did_wake_up::SystemDidWakeUpEvent,
};

pub enum EventReceived {
    ApplicationDidLaunch(ApplicationMonitorEvent),
    ApplicationDidTerminate(ApplicationMonitorEvent),
    DeviceDidConnect(DeviceDidConnectEvent),
    DeviceDidDisconnect(DeviceDidDisconnectEvent),
    KeyDown(KeyDownEvent),
    SystemDidWakeUp(SystemDidWakeUpEvent),
    UnknownEvent(String),
    EventDeserializationError(String),
}

impl EventReceived {
    pub fn from_json(json: &str) -> Result<EventReceived> {
        let event_base: EventBase = serde_json::from_str(json)?;

        match event_base.event.as_str() {
            StreamDeckEventTitle::APPLICATION_DID_LAUNCH => {
                match serde_json::from_str::<ApplicationMonitorEvent>(json) {
                    Ok(event) => Ok(EventReceived::ApplicationDidLaunch(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(e.to_string())),
                }
            }
            StreamDeckEventTitle::APPLICATION_DID_TERMINATE => Ok(
                EventReceived::ApplicationDidTerminate(serde_json::from_str(json)?),
            ),
            StreamDeckEventTitle::DEVICE_DID_CONNECT => {
                Ok(EventReceived::DeviceDidConnect(serde_json::from_str(json)?))
            }
            StreamDeckEventTitle::DEVICE_DID_DISCONNECT => Ok(EventReceived::DeviceDidDisconnect(
                serde_json::from_str(json)?,
            )),
            StreamDeckEventTitle::SYSTEM_DID_WAKE_UP => {
                Ok(EventReceived::SystemDidWakeUp(serde_json::from_str(json)?))
            }
            StreamDeckEventTitle::KEY_DOWN => match serde_json::from_str::<KeyDownEvent>(json) {
                Ok(event) => Ok(EventReceived::KeyDown(event)),
                Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                    "{}, {}",
                    e.to_string(),
                    &json
                ))),
            },
            _ => Ok(EventReceived::UnknownEvent(event_base.event)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventBase {
    pub event: String,
}
