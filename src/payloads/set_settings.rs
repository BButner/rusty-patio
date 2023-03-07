use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckSetSettingsMessage<T: Sized + serde::Serialize> {
    pub event: String,
    pub context: String,
    pub payload: T,
}
