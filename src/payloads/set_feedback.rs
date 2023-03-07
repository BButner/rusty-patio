use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetFeedbackMessage {
    event: String,
    context: String,
    payload: HashMap<String, Value>,
}

impl StreamDeckSetFeedbackMessage {
    pub fn new(context: String, payload: HashMap<String, Value>) -> Self {
        Self {
            event: StreamDeckEventTitle::SET_FEEDBACK.to_string(),
            context,
            payload,
        }
    }
}
