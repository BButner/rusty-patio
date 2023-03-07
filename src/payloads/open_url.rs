use serde::{Deserialize, Serialize};

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckOpenUrlMessage {
    event: String,
    payload: StreamDeckOpenUrlPayload,
}

impl StreamDeckOpenUrlMessage {
    pub fn new(url: String) -> Self {
        Self {
            event: StreamDeckEventTitle::OPEN_URL.to_string(),
            payload: StreamDeckOpenUrlPayload { url },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckOpenUrlPayload {
    url: String,
}
