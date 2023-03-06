use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct TouchTapEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: TouchTapEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchTapEventPayload {
    pub settings: HashMap<String, Value>,
    pub coordinates: Coordinates,
    pub tap_pos: [u8; 2],
    pub hold: bool,
}
