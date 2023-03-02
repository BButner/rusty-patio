use serde::{Deserialize, Serialize};

use crate::streamdeck::events::EVENT_LOG_MESSAGE;

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckLogMessage {
    event: String,
    payload: StreamDeckLogMessagePayload,
}

impl StreamDeckLogMessage {
    pub fn new(message: String) -> Self {
        StreamDeckLogMessage {
            event: EVENT_LOG_MESSAGE.to_string(),
            payload: StreamDeckLogMessagePayload { message },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckLogMessagePayload {
    message: String,
}
