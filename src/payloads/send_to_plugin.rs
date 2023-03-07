use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSendToPluginMessage {
    pub action: String,
    pub event: String,
    pub context: String,
    pub payload: HashMap<String, Value>,
}

impl StreamDeckSendToPluginMessage {
    pub fn new(action: String, context: String, payload: HashMap<String, Value>) -> Self {
        Self {
            action,
            event: StreamDeckEventTitle::SEND_TO_PLUGIN.to_string(),
            context,
            payload,
        }
    }
}
