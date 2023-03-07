use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckEventContextMessage {
    pub event: String,
    pub context: String,
}
