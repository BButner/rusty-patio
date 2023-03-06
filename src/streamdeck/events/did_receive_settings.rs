use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct DidReceiveSettingsEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: DidReceiveSettingsEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveSettingsEventPayload {
    settings: HashMap<String, Value>,
    coordinates: Coordinates,
    is_in_multi_action: bool,
    state: Option<u8>,
}
