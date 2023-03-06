use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: KeyEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyEventPayload {
    coordinates: Coordinates,
    state: Option<u8>,
    user_desired_state: Option<u8>,
    is_in_multi_action: bool,
    settings: HashMap<String, Value>,
}
