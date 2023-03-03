use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

use crate::{payloads::register::StreamDeckPluginRegister, streamdeck::arguments::StreamDeckArgs};

pub async fn connect_streamdeck(args: &StreamDeckArgs) {
    let (mut ws_stream, _) = connect_async(format!("ws://localhost:{}", args.port))
        .await
        .unwrap();

    let (mut write, read) = ws_stream.split();

    tokio::spawn(async move {
        read.for_each(|message| async {
            let message_bytes = message.unwrap().into_data();
            handle_message(message_bytes);
        })
        .await;
    });

    register_plugin(&mut write, &args.register_event, &args.plugin_uuid).await;
}

fn handle_message(message_bytes: Vec<u8>) {
    let message = String::from_utf8(message_bytes).unwrap();
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
