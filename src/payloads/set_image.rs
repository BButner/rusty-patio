use serde::{Deserialize, Serialize};

use crate::streamdeck::events::event_title::StreamDeckEventTitle;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetImageMessage {
    pub event: String,
    pub context: String,
    pub payload: StreamDeckSetImageMessagePayload,
}

impl StreamDeckSetImageMessage {
    pub fn new(context: String, image: String, target: u8, state: Option<u8>) -> Self {
        Self {
            event: StreamDeckEventTitle::SET_IMAGE.to_string(),
            context,
            payload: StreamDeckSetImageMessagePayload {
                image,
                target,
                state,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetImageMessagePayload {
    pub image: String,
    pub target: u8,
    pub state: Option<u8>,
}
