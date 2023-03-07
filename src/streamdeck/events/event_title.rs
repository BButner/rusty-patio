pub struct StreamDeckEventTitle;

impl StreamDeckEventTitle {
    pub const APPLICATION_DID_LAUNCH: &str = "applicationDidLaunch";
    pub const APPLICATION_DID_TERMINATE: &str = "applicationDidTerminate";
    pub const DEVICE_DID_CONNECT: &str = "deviceDidConnect";
    pub const DEVICE_DID_DISCONNECT: &str = "deviceDidDisconnect";
    pub const DIAL_PRESS: &str = "dialPress";
    pub const DIAL_ROTATE: &str = "dialRotate";
    pub const DID_RECEIVE_GLOBAL_SETTINGS: &str = "didReceiveGlobalSettings";
    pub const DID_RECEIVE_SETTINGS: &str = "didReceiveSettings";
    pub const GET_GLOBAL_SETTINGS: &str = "getGlobalSettings";
    pub const GET_SETTINGS: &str = "getSettings";
    pub const KEY_DOWN: &str = "keyDown";
    pub const KEY_UP: &str = "keyUp";
    pub const LOG_MESSAGE: &str = "logMessage";
    pub const OPEN_URL: &str = "openUrl";
    pub const PI_DID_APPEAR: &str = "propertyInspectorDidAppear";
    pub const PI_DID_DISAPPEAR: &str = "propertyInspectorDidDisappear";
    pub const SEND_TO_PLUGIN: &str = "sendToPlugin";
    pub const SEND_TO_PROPERTY_INSPECTOR: &str = "sendToPropertyInspector";
    pub const SET_FEEDBACK: &str = "setFeedback";
    pub const SET_FEEDBACK_LAYOUT: &str = "setFeedbackLayout";
    pub const SET_GLOBAL_SETTINGS: &str = "setGlobalSettings";
    pub const SET_IMAGE: &str = "setImage";
    pub const SET_SETTINGS: &str = "setSettings";
    pub const SET_STATE: &str = "setState";
    pub const SET_TITLE: &str = "setTitle";
    pub const SHOW_ALERT: &str = "showAlert";
    pub const SHOW_OK: &str = "showOk";
    pub const SWITCH_TO_PROFILE: &str = "switchToProfile";
    pub const SYSTEM_DID_WAKE_UP: &str = "systemDidWakeUp";
    pub const TITLE_PARAMETERS_DID_CHANGE: &str = "titleParametersDidChange";
    pub const TOUCH_TAP: &str = "touchTap";
    pub const WILL_APPEAR: &str = "willAppear";
    pub const WILL_DISAPPEAR: &str = "willDisappear";
}
