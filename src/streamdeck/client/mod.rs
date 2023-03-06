use anyhow::Result;
use serde::Serialize;

use crate::payloads::{
    log_message::StreamDeckLogMessage, register::StreamDeckPluginRegister,
    set_image::StreamDeckSetImageMessage,
};

use super::{events::event_received::EventReceived, generic::StreamDeckTarget};

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

    async fn send_message(&mut self, message: String) -> Result<()> {
        self.transmit_message.send(message).map_err(|e| e.into())
    }

    async fn send_json_message<T: Sized + Serialize>(&mut self, message: T) -> Result<()> {
        self.send_message(serde_json::to_string(&message).unwrap())
            .await
    }

    pub async fn register_plugin(&mut self, event: &String, uuid: &String) {
        self.send_json_message(StreamDeckPluginRegister::new(event, uuid))
            .await
            .unwrap();
    }

    pub async fn log_message(&mut self, message: String) -> Result<()> {
        self.send_json_message(StreamDeckLogMessage::new(message))
            .await
    }

    pub async fn set_image(
        &mut self,
        context: String,
        image: String,
        target: StreamDeckTarget,
        state: Option<u8>,
    ) -> Result<()> {
        self.send_json_message(StreamDeckSetImageMessage::new(
            context, image, target, state,
        ))
        .await
    }
}
