use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckPluginRegister {
    pub event: String,
    pub uuid: String,
}
