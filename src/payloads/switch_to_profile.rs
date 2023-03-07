use serde::{Deserialize, Serialize};

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckSwitchToProfileMessage {
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: StreamDeckSwitchToProfilePayload,
}

impl StreamDeckSwitchToProfileMessage {
    pub fn new(context: String, device: String, profile: String) -> Self {
        Self {
            event: StreamDeckEventTitle::SWITCH_TO_PROFILE.to_string(),
            context,
            device,
            payload: StreamDeckSwitchToProfilePayload { profile },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckSwitchToProfilePayload {
    pub profile: String,
}
