use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct DialPressEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: DialPressEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DialPressEventPayload {
    pub settings: HashMap<String, Value>,
    pub coordinates: Coordinates,
    pub pressed: bool,
}
