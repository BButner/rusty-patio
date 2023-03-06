use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppearEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: AppearEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearEventPayload {
    controller: String,
    settings: HashMap<String, Value>,
    coordinates: Coordinates,
    state: Option<u8>,
    is_in_multi_action: bool,
}
