use serde::Serialize;

use crate::payloads::{log_message::StreamDeckLogMessage, register::StreamDeckPluginRegister};

use super::events::event_received::EventReceived;

pub struct StreamDeckClient {
    pub received_events: tokio::sync::mpsc::UnboundedReceiver<EventReceived>,
    transmit_message: tokio::sync::mpsc::UnboundedSender<String>,
}

impl StreamDeckClient {
    pub fn new(
        received_events: tokio::sync::mpsc::UnboundedReceiver<EventReceived>,
        transmit_message: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> Self {
        StreamDeckClient {
            received_events,
            transmit_message,
        }
    }

    async fn send_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        self.transmit_message.send(message);

        Ok(())
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
