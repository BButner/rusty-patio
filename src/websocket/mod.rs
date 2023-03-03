use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

use crate::{
    payloads::register::StreamDeckPluginRegister,
    streamdeck::{
        arguments::StreamDeckArgs,
        client::StreamDeckClient,
        events::{event_received::EventReceived, message_sent::StreamDeckMessage},
    },
};

pub async fn connect_streamdeck(
    args: &StreamDeckArgs,
) -> Result<StreamDeckClient, Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(format!("ws://localhost:{}", args.port))
        .await
        .unwrap();

    let (mut write, read) = ws_stream.split();

    let (tx, rx) = futures_channel::mpsc::unbounded::<EventReceived>();
    let (tx_message, rx_message) = futures_channel::mpsc::unbounded::<String>();

    tokio::spawn(async move {
        read.for_each(|message| async {
            let message_bytes = message.unwrap().into_data();
            handle_message(message_bytes, &tx);
        })
        .await;
    });

    let stdin_to_ws = rx_message.map(|f| Ok(Message::text(f))).forward(write);

    tokio::spawn(async move {
        stdin_to_ws.await.unwrap();
    });

    let mut client = StreamDeckClient::new(rx, tx_message);

    client
        .register_plugin(&args.register_event, &args.plugin_uuid)
        .await;

    Ok(client)
}

fn handle_message(
    message_bytes: Vec<u8>,
    tx: &futures_channel::mpsc::UnboundedSender<EventReceived>,
) {
    let message = String::from_utf8(message_bytes).unwrap();

    if let Ok(event) = EventReceived::from_json(&message) {
        tx.unbounded_send(event).unwrap();
    }
}
