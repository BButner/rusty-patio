use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    appear::AppearEvent, application_monitor_event::ApplicationMonitorEvent,
    device_did_connect::DeviceDidConnectEvent, device_did_disconnect::DeviceDidDisconnectEvent,
    dial_press::DialPressEvent, dial_rotate::DialRotateEvent,
    did_receive_global_settings::DidReceiveGlobalSettingsEvent, event_title::StreamDeckEventTitle,
    key::KeyEvent, pi_appear::PIAppearEvent, send_to_plugin::SendToPluginEvent,
    system_did_wake_up::SystemDidWakeUpEvent,
    title_parameters_did_change::TitleParametersDidChangeEvent, touch_tap::TouchTapEvent,
};

pub enum EventReceived {
    ApplicationDidLaunch(ApplicationMonitorEvent),
    ApplicationDidTerminate(ApplicationMonitorEvent),
    DeviceDidConnect(DeviceDidConnectEvent),
    DeviceDidDisconnect(DeviceDidDisconnectEvent),
    DialPress(DialPressEvent),
    DialRotate(DialRotateEvent),
    DidReceiveSettings(DidReceiveGlobalSettingsEvent),
    DidReceiveGlobalSettings(DidReceiveGlobalSettingsEvent),
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    PropertyInspectorDidAppear(PIAppearEvent),
    PropertyInspectorDidDisappear(PIAppearEvent),
    SendToPlugin(SendToPluginEvent),
    SystemDidWakeUp(SystemDidWakeUpEvent),
    TitleParametersDidChange(TitleParametersDidChangeEvent),
    TouchTap(TouchTapEvent),
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
            StreamDeckEventTitle::DIAL_PRESS => {
                match serde_json::from_str::<DialPressEvent>(json) {
                    Ok(event) => Ok(EventReceived::DialPress(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::DIAL_ROTATE => {
                match serde_json::from_str::<DialRotateEvent>(json) {
                    Ok(event) => Ok(EventReceived::DialRotate(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::DID_RECEIVE_SETTINGS => {
                match serde_json::from_str::<DidReceiveGlobalSettingsEvent>(json) {
                    Ok(event) => Ok(EventReceived::DidReceiveSettings(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::DID_RECEIVE_GLOBAL_SETTINGS => {
                match serde_json::from_str::<DidReceiveGlobalSettingsEvent>(json) {
                    Ok(event) => Ok(EventReceived::DidReceiveGlobalSettings(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::PI_DID_APPEAR => {
                match serde_json::from_str::<PIAppearEvent>(json) {
                    Ok(event) => Ok(EventReceived::PropertyInspectorDidAppear(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::PI_DID_DISAPPEAR => {
                match serde_json::from_str::<PIAppearEvent>(json) {
                    Ok(event) => Ok(EventReceived::PropertyInspectorDidDisappear(event)),
                    Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                        "{}, {}",
                        e.to_string(),
                        &json
                    ))),
                }
            }
            StreamDeckEventTitle::SEND_TO_PLUGIN => {
                match serde_json::from_str::<SendToPluginEvent>(json) {
                    Ok(event) => Ok(EventReceived::SendToPlugin(event)),
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
            StreamDeckEventTitle::TOUCH_TAP => match serde_json::from_str::<TouchTapEvent>(json) {
                Ok(event) => Ok(EventReceived::TouchTap(event)),
                Err(e) => Ok(EventReceived::EventDeserializationError(format!(
                    "{}, {}",
                    e.to_string(),
                    &json
                ))),
            },
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
