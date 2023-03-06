use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct TouchTapEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: TouchTapEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouchTapEventPayload {
    settings: HashMap<String, Value>,
    coordinates: Coordinates,
    tap_pos: [u8; 2],
    hold: bool,
}
