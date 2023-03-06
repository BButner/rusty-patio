use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDownEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: KeyDownEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDownEventPayload {
    coordinates: Coordinates,
    state: Option<u8>,
    #[serde(rename = "userDesiredState")]
    user_desired_state: Option<u8>,
    #[serde(rename = "isInMultiAction")]
    is_in_multi_action: bool,
    settings: HashMap<String, Value>,
}
