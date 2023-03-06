use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialRotateEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: DialRotateEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialRotateEventPayload {
    settings: HashMap<String, Value>,
    coordinates: Coordinates,
    ticks: i32,
    pressed: bool,
}
