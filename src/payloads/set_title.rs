use serde::{Deserialize, Serialize};

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetTitleMessage {
    pub event: String,
    pub context: String,
    pub payload: SetTitleMessagePayload,
}

impl StreamDeckSetTitleMessage {
    pub fn new(context: String, title: String, target: u8, state: Option<u8>) -> Self {
        Self {
            event: StreamDeckEventTitle::SET_TITLE.to_string(),
            context,
            payload: SetTitleMessagePayload {
                title,
                target,
                state,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTitleMessagePayload {
    pub title: String,
    pub target: u8,
    pub state: Option<u8>,
}
