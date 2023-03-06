pub struct StreamDeckEventTitle;

impl StreamDeckEventTitle {
    pub const APPLICATION_DID_LAUNCH: &str = "applicationDidLaunch";
    pub const APPLICATION_DID_TERMINATE: &str = "applicationDidTerminate";
    pub const LOG_MESSAGE: &str = "logMessage";
    pub const DEVICE_DID_CONNECT: &str = "deviceDidConnect";
    pub const DEVICE_DID_DISCONNECT: &str = "deviceDidDisconnect";
    pub const KEY_DOWN: &str = "keyDown";
    pub const SYSTEM_DID_WAKE_UP: &str = "systemDidWakeUp";
}
