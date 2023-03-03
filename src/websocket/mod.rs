use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

use crate::{
    payloads::register::StreamDeckPluginRegister,
    streamdeck::{arguments::StreamDeckArgs, client::StreamDeckClient},
};

pub async fn connect_streamdeck(
    args: &StreamDeckArgs,
) -> Result<StreamDeckClient, Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(format!("ws://localhost:{}", args.port))
        .await
        .unwrap();

    let (mut write, read) = ws_stream.split();

    let (tx, rx) = futures_channel::mpsc::unbounded::<String>();

    tokio::spawn(async move {
        read.for_each(|message| async {
            let message_bytes = message.unwrap().into_data();
            handle_message(message_bytes, &tx);
        })
        .await;
    });

    register_plugin(&mut write, &args.register_event, &args.plugin_uuid).await;

    Ok(StreamDeckClient {
        received_events: rx,
    })
}

fn handle_message(message_bytes: Vec<u8>, tx: &futures_channel::mpsc::UnboundedSender<String>) {
    let message = String::from_utf8(message_bytes).unwrap();

    tx.unbounded_send(message).unwrap();
}

async fn register_plugin(
    sink: &mut SplitSink<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message,
    >,
    event: &String,
    uuid: &String,
) {
    let payload = serde_json::to_string(&StreamDeckPluginRegister {
        event: event.clone(),
        uuid: uuid.clone(),
    })
    .unwrap();

    sink.send(Message::text(payload)).await.unwrap();
}
