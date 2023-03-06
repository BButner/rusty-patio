use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemDidWakeUpEvent {
    pub event: String,
}
