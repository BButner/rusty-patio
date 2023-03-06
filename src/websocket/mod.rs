use futures_util::{SinkExt, StreamExt};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::streamdeck::{
    arguments::StreamDeckArgs, client::StreamDeckClient, events::event_received::EventReceived,
};

pub async fn connect_streamdeck(
    args: &StreamDeckArgs,
) -> Result<StreamDeckClient, Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(format!("ws://localhost:{}", args.port))
        .await
        .unwrap();

    let (mut write, mut read) = ws_stream.split();

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<EventReceived>();
    let (tx_message, mut rx_message) = tokio::sync::mpsc::unbounded_channel::<String>();

    tokio::spawn(async move {
        while let Some(Ok(message)) = read.next().await {
            let message_bytes = message.into_data();
            handle_message(message_bytes, tx.clone());
        }
    });

    tokio::spawn(async move {
        while let Some(message) = rx_message.recv().await {
            write.send(Message::text(message)).await.unwrap();
        }
    });

    let mut client = StreamDeckClient::new(rx, tx_message);

    client
        .register_plugin(&args.register_event, &args.plugin_uuid)
        .await;

    Ok(client)
}

fn handle_message(message_bytes: Vec<u8>, tx: tokio::sync::mpsc::UnboundedSender<EventReceived>) {
    let message = String::from_utf8(message_bytes).unwrap();

    match EventReceived::from_json(&message) {
        Ok(event) => match tx.send(event) {
            Ok(_) => {}
            Err(_) => {}
        },
        Err(_) => {}
    }
}
