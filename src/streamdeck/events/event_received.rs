use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    appear::AppearEvent, application_monitor_event::ApplicationMonitorEvent,
    device_did_connect::DeviceDidConnectEvent, device_did_disconnect::DeviceDidDisconnectEvent,
    event_title::StreamDeckEventTitle, key::KeyEvent, system_did_wake_up::SystemDidWakeUpEvent,
    title_parameters_did_change::TitleParametersDidChangeEvent,
};

pub enum EventReceived {
    ApplicationDidLaunch(ApplicationMonitorEvent),
    ApplicationDidTerminate(ApplicationMonitorEvent),
    DeviceDidConnect(DeviceDidConnectEvent),
    DeviceDidDisconnect(DeviceDidDisconnectEvent),
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    SystemDidWakeUp(SystemDidWakeUpEvent),
    TitleParametersDidChange(TitleParametersDidChangeEvent),
    UnknownEvent(String),
    EventDeserializationError(String),
    WillAppear(AppearEvent),
    WillDisappear(AppearEvent),
}

impl EventReceived {
    pub fn from_json(json: &str) -> Result<EventReceived> {
        let event_base: EventBase = serde_json::from_str(json)?;

        match event_base.event.as_str() {
            StreamDeckEventTitle::APPLICATION_DID_LAUNCH => {
                match serde_json::from_str::<ApplicationMonitorEvent>(json) {
                    Ok(event) => Ok(EventReceived::ApplicationDidLaunch(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::APPLICATION_DID_TERMINATE => {
                match serde_json::from_str::<ApplicationMonitorEvent>(json) {
                    Ok(event) => Ok(EventReceived::ApplicationDidTerminate(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::DEVICE_DID_CONNECT => {
                match serde_json::from_str::<DeviceDidConnectEvent>(json) {
                    Ok(event) => Ok(EventReceived::DeviceDidConnect(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::DEVICE_DID_DISCONNECT => {
                match serde_json::from_str::<DeviceDidDisconnectEvent>(json) {
                    Ok(event) => Ok(EventReceived::DeviceDidDisconnect(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }

            StreamDeckEventTitle::SYSTEM_DID_WAKE_UP => {
                match serde_json::from_str::<SystemDidWakeUpEvent>(json) {
                    Ok(event) => Ok(EventReceived::SystemDidWakeUp(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::TITLE_PARAMETERS_DID_CHANGE => {
                match serde_json::from_str::<TitleParametersDidChangeEvent>(json) {
                    Ok(event) => Ok(EventReceived::TitleParametersDidChange(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::KEY_DOWN => match serde_json::from_str::<KeyEvent>(json) {
                Ok(event) => Ok(EventReceived::KeyDown(event)),
                Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                    "{}, {}",
                    e.to_string(),
                    &json
                ))),
            },
            StreamDeckEventTitle::KEY_UP => match serde_json::from_str::<KeyEvent>(json) {
                Ok(event) => Ok(EventReceived::KeyUp(event)),
                Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                    "{}, {}",
                    e.to_string(),
                    &json
                ))),
            },
            StreamDeckEventTitle::WILL_APPEAR => match serde_json::from_str::<AppearEvent>(json) {
                Ok(event) => Ok(EventReceived::WillAppear(event)),
                Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                    "{}, {}",
                    e.to_string(),
                    &json
                ))),
            },
            StreamDeckEventTitle::WILL_DISAPPEAR => match serde_json::from_str::<AppearEvent>(json)
            {
                Ok(event) => Ok(EventReceived::WillDisappear(event)),
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
