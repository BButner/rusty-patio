use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialRotateEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: DialRotateEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialRotateEventPayload {
    pub settings: HashMap<String, Value>,
    pub coordinates: Coordinates,
    pub ticks: i32,
    pub pressed: bool,
}
