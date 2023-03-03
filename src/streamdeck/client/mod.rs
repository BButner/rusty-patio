pub struct StreamDeckClient {
    pub received_events: futures_channel::mpsc::UnboundedReceiver<String>,
}
