use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetSettingsMessage<T: Sized + serde::Serialize> {
    event: String,
    context: String,
    payload: T,
}

impl<T: Sized + serde::Serialize> StreamDeckSetSettingsMessage<T> {
    pub fn new(context: String, payload: T) -> Self {
        Self {
            event: "setSettings".to_string(),
            context,
            payload,
        }
    }
}
