use serde::{Deserialize, Serialize};

pub mod device_info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub column: u8,
    pub row: u8,
}
