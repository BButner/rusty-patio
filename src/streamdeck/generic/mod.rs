use serde::{Deserialize, Serialize};

pub mod device_info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub column: u8,
    pub row: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamDeckTarget;

impl StreamDeckTarget {
    pub const HARDWARE_AND_SOFTWARE: u8 = 0;
    pub const HARDWARE: u8 = 1;
    pub const SOFTWARE: u8 = 2;
}
