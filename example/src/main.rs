use std::collections::HashMap;

use rusty_patio::{
    streamdeck::{
        arguments::StreamDeckArgs, events::event_received::EventReceived, generic::StreamDeckTarget,
    },
    websocket::connect_streamdeck,
};

#[tokio::main(worker_threads = 1)]
async fn main() {
    let args = StreamDeckArgs::new();
    let client = connect_streamdeck(&args).await;
    let mut buttons: HashMap<String, i64> = HashMap::new();

    if let Ok(mut client) = client {
        while let Some(event) = client.received_events.recv().await {
            match event {
                EventReceived::WillAppear(event) => {
                    if !buttons.contains_key(&event.context) {
                        buttons.insert(event.context.clone(), 0);
                        client
                            .set_title(
                                event.context.clone(),
                                "0".to_string(),
                                StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                                None,
                            )
                            .await
                            .unwrap();
                    }
                }
                EventReceived::KeyDown(event) => {
                    if let Some(count) = buttons.get_mut(&event.context) {
                        *count += 1;
                        client
                            .set_title(
                                event.context.clone(),
                                count.to_string().clone(),
                                StreamDeckTarget::HARDWARE_AND_SOFTWARE,
                                None,
                            )
                            .await
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
