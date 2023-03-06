use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleParametersDidChangeEvent {
    pub action: String,
    pub event: String,
    pub context: String,
    pub device: String,
    pub payload: TitleParametersDidChangeEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleParametersDidChangeEventPayload {
    pub coordinates: Coordinates,
    pub settings: HashMap<String, Value>,
    pub state: u8,
    pub title: String,
    pub title_parameters: TitleParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleParameters {
    pub font_family: String,
    pub font_size: u8,
    pub font_style: String,
    pub font_underline: bool,
    pub show_title: bool,
    pub title_alignment: String,
    pub title_color: String,
}
