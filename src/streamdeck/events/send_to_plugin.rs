use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendToPluginEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub payload: HashMap<String, Value>,
}
