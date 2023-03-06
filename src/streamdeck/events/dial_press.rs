use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialPressEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: DialPressEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialPressEventPayload {
    settings: HashMap<String, Value>,
    coordinates: Coordinates,
    pressed: bool,
}
