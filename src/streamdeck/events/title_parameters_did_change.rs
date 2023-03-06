use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::streamdeck::generic::Coordinates;

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleParametersDidChangeEvent {
    action: String,
    event: String,
    context: String,
    device: String,
    payload: TitleParametersDidChangeEventPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleParametersDidChangeEventPayload {
    coordinates: Coordinates,
    settings: HashMap<String, Value>,
    state: u8,
    title: String,
    title_parameters: TitleParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleParameters {
    font_family: String,
    font_size: u8,
    font_style: String,
    font_underline: bool,
    show_title: bool,
    title_alignment: String,
    title_color: String,
}
