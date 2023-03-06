use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationMonitorEvent {
    pub event: String,
    pub payload: ApplicationMonitorEventPayload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationMonitorEventPayload {
    pub application: String,
}
