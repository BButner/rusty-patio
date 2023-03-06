use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct DidReceiveSettingsEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: DidReceiveSettingsEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidReceiveSettingsEventPayload {
    pub settings: HashMap<String, Value>,
    pub coordinates: Coordinates,
    pub is_in_multi_action: bool,
    pub state: Option<u8>,
}
