use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct StreamDeckArgs {
    pub port: i32,
    pub plugin_uuid: String,
    pub register_event: String,
    pub info: StreamDeckInfo,
}

impl StreamDeckArgs {
    pub fn new() -> Self {
        get_args()
    }
}

fn get_args() -> StreamDeckArgs {
    let mut port = 0;
    let mut plugin_uuid = String::new();
    let mut register_event = String::new();
    let mut info: Option<StreamDeckInfo> = None;

    let args = std::env::args().collect::<Vec<String>>();

    for i in (0..args.len()).skip(1).step_by(2) {
        if args[i].starts_with("-port") {
            port = args[i + 1].parse().unwrap();
        } else if args[i].starts_with("-pluginUUID") {
            plugin_uuid = args[i + 1].to_string();
        } else if args[i].starts_with("-registerEvent") {
            register_event = args[i + 1].to_string();
        } else if args[i].starts_with("-info") {
            info = Some(serde_json::from_str(&args[i + 1]).unwrap());
        }
    }

    StreamDeckArgs {
        port,
        plugin_uuid,
        register_event,
        info: info.unwrap(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckInfo {
    pub application: StreamDeckApplication,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckApplication {
    pub font: String,
    pub language: String,
    pub platform: String,
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckColors {
    #[serde(rename = "buttonMouseOverBackgroundColor")]
    pub button_mouse_over_background_color: String,
    #[serde(rename = "buttonPressedBackgroundColor")]
    pub button_pressed_background_color: String,
    #[serde(rename = "buttonPressedBorderColor")]
    pub button_pressed_border_color: String,
    #[serde(rename = "buttonPressedTextColor")]
    pub button_pressed_text_color: String,
    #[serde(rename = "highlightColor")]
    pub highlight_color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckDevice {
    pub id: String,
    pub name: String,
    pub size: StreamDeckSize,
    #[serde(rename = "type")]
    pub device_type: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckSize {
    pub columns: i32,
    pub rows: i32,
}
