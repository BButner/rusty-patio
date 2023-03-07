use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckGetSettingsMessage {
    pub event: String,
    pub context: String,
}
