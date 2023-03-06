use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PIAppearEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
}
