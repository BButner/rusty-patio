use serde::{Deserialize, Serialize};

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckLogMessage {
    event: String,
    payload: StreamDeckLogMessagePayload,
}

impl StreamDeckLogMessage {
    pub fn new(message: String) -> Self {
        StreamDeckLogMessage {
            event: StreamDeckEventTitle::LogMessage.to_string(),
            payload: StreamDeckLogMessagePayload { message },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckLogMessagePayload {
    message: String,
}
