use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DidReceiveGlobalSettingsEvent {
    pub event: String,
    pub payload: DidReceiveGlobalSettingsEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DidReceiveGlobalSettingsEventPayload {
    pub settings: HashMap<String, Value>,
}
