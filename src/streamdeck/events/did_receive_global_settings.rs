use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DidReceiveGlobalSettingsEvent {
    event: String,
    payload: DidReceiveGlobalSettingsEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DidReceiveGlobalSettingsEventPayload {
    settings: HashMap<String, Value>,
}
