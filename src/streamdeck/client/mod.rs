use futures_util::{stream::SplitSink, SinkExt};
use serde::Serialize;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use crate::payloads::{log_message::StreamDeckLogMessage, register::StreamDeckPluginRegister};

use super::events::{event_received::EventReceived, message_sent::StreamDeckMessage};

pub struct StreamDeckClient {
    pub received_events: futures_channel::mpsc::UnboundedReceiver<EventReceived>,
    transmit_message: futures_channel::mpsc::UnboundedSender<String>,
}

impl StreamDeckClient {
    pub fn new(
        received_events: futures_channel::mpsc::UnboundedReceiver<EventReceived>,
        transmit_message: futures_channel::mpsc::UnboundedSender<String>,
    ) -> Self {
        StreamDeckClient {
            received_events,
            transmit_message,
        }
    }

    async fn send_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        self.transmit_message
            .send(message)
            .await
            .map_err(|e| e.into())
    }

    async fn send_json_message<T: Sized + Serialize>(
        &mut self,
        message: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message(serde_json::to_string(&message).unwrap())
            .await
    }

    pub async fn register_plugin(&mut self, event: &String, uuid: &String) {
        self.send_json_message(StreamDeckPluginRegister::new(event, uuid))
            .await
            .unwrap();
    }

    pub async fn log_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        self.send_json_message(StreamDeckLogMessage::new(message))
            .await
    }
}
