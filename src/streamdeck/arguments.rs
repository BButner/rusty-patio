use serde::{Deserialize, Serialize};

/// The arguments passed to the plugin by the Stream Deck application.
#[derive(Debug)]
pub struct StreamDeckArgs {
    /// The port to connect to the Stream Deck application.
    pub port: i32,
    /// The UUID of the plugin.
    pub plugin_uuid: String,
    /// The event to register with the Stream Deck application.
    pub register_event: String,
    /// The info about the Stream Deck application.
    pub info: StreamDeckInfo,
}

/// The arguments passed to the plugin by the Stream Deck application.
impl StreamDeckArgs {
    /// Creates a new instance of the arguments.
    pub fn new() -> Self {
        get_args()
    }
}

/// Gets the arguments passed to the plugin by the Stream Deck application.
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

/// The info about the Stream Deck application.
#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckInfo {
    /// The general information about the Stream Deck application.
    pub application: StreamDeckApplication,
}

/// The info about the Stream Deck application.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamDeckApplication {
    /// The font being used in the application.
    pub font: String,
    /// The language being used in the application.
    pub language: String,
    /// The current platform. Possible values are `mac` and `windows`.
    pub platform: String,
    /// The version of the platform.
    pub platform_version: String,
    /// The version of the Stream Deck application.
    pub version: String,
}

/// Preferred user colors.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamDeckColors {
    /// The background color of the button on mouse over.
    pub button_mouse_over_background_color: String,
    /// The background color of the button while pressed.
    pub button_pressed_background_color: String,
    /// The border color fo the button while pressed.
    pub button_pressed_border_color: String,
    /// The text color of the button while pressed.
    pub button_pressed_text_color: String,
    /// The highlight color.
    pub highlight_color: String,
}

/// Information about the Elgato Stream Deck device.
#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckDevice {
    /// The unique ID of the Device.
    pub id: String,
    /// The name of the device.
    pub name: String,
    /// The size of the device.
    pub size: StreamDeckSize,
    /// The type of the device.
    #[serde(rename = "type")]
    pub device_type: i32,
}

/// The size of the device.
#[derive(Serialize, Deserialize, Debug)]
pub struct StreamDeckSize {
    /// The number of columns of buttons.
    pub columns: i32,
    /// The number of rows of buttons.
    pub rows: i32,
}
