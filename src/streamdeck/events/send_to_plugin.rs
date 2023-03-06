use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendToPluginEvent {
    action: String,
    event: String,
    context: String,
    payload: HashMap<String, Value>,
}
