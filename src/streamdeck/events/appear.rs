use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppearEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: AppearEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearEventPayload {
    pub controller: String,
    pub settings: HashMap<String, Value>,
    pub coordinates: Coordinates,
    pub state: Option<u8>,
    pub is_in_multi_action: bool,
}
