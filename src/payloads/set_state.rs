use serde::{Deserialize, Serialize};

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetStateMessage {
    pub event: String,
    pub context: String,
    pub payload: StreamDeckSetStatePayload,
}

impl StreamDeckSetStateMessage {
    pub fn new(context: String, state: u8) -> Self {
        Self {
            event: StreamDeckEventTitle::SET_STATE.to_string(),
            context,
            payload: StreamDeckSetStatePayload { state },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetStatePayload {
    state: u8,
}
