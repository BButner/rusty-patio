use serde::{Deserialize, Serialize};

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetFeedbackLayoutMessage {
    event: String,
    context: String,
    payload: StreamDeckSetFeedbackLayoutPayload,
}

impl StreamDeckSetFeedbackLayoutMessage {
    pub fn new(context: String, layout: String) -> Self {
        Self {
            event: StreamDeckEventTitle::SET_FEEDBACK.to_string(),
            context,
            payload: StreamDeckSetFeedbackLayoutPayload { layout },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetFeedbackLayoutPayload {
    layout: String,
}
