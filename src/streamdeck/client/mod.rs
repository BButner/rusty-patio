use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

use crate::payloads::{
    log_message::StreamDeckLogMessage, open_url::StreamDeckOpenUrlMessage,
    register::StreamDeckPluginRegister, set_image::StreamDeckSetImageMessage,
    set_settings::StreamDeckSetSettingsMessage, set_title::StreamDeckSetTitleMessage,
};

use super::{
    events::{event_received::EventReceived, event_title::StreamDeckEventTitle},
    generic::StreamDeckTarget,
};

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
        image_path: String,
        target: StreamDeckTarget,
        state: Option<u8>,
    ) -> Result<()> {
        let image_bytes = tokio::fs::read(&image_path).await?;
        let encoded = general_purpose::STANDARD.encode(&image_bytes);
        let mime_type = mime_guess::from_path(&image_path).first();

        if let Some(mime_type) = mime_type {
            self.send_json_message(StreamDeckSetImageMessage::new(
                context,
                format!("data:{};base64,{}", mime_type, encoded),
                target,
                state,
            ))
            .await
        } else {
            Err(anyhow::anyhow!("Could not determine mime type"))
        }
    }

    pub async fn set_title(
        &mut self,
        context: String,
        title: String,
        target: u8,
        state: Option<u8>,
    ) -> Result<()> {
        self.send_json_message(StreamDeckSetTitleMessage::new(
            context, title, target, state,
        ))
        .await
    }

    pub async fn set_settings<T: Sized + Serialize>(
        &mut self,
        context: String,
        payload: T,
    ) -> Result<()> {
        self.send_json_message(StreamDeckSetSettingsMessage {
            event: StreamDeckEventTitle::SET_SETTINGS.to_string(),
            context,
            payload,
        })
        .await
    }

    pub async fn set_global_settings<T: Sized + Serialize>(
        &mut self,
        context: String,
        payload: T,
    ) -> Result<()> {
        self.send_json_message(StreamDeckSetSettingsMessage {
            event: StreamDeckEventTitle::SET_GLOBAL_SETTINGS.to_string(),
            context,
            payload,
        })
        .await
    }

    pub async fn open_url(&mut self, url: String) -> Result<()> {
        self.send_json_message(StreamDeckOpenUrlMessage::new(url))
            .await
    }
}
