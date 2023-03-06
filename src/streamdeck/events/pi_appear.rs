use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PIAppearEvent {
    action: String,
    event: String,
    context: String,
    device: String,
}
