use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckPluginRegister {
    pub event: String,
    pub uuid: String,
}

impl StreamDeckPluginRegister {
    pub fn new(event: &String, uuid: &String) -> Self {
        StreamDeckPluginRegister {
            event: event.to_string(),
            uuid: uuid.to_string(),
        }
    }
}
