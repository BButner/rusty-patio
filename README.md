# Rusty Patio (Unofficial Rust Stream Deck SDK)

[![crates.io](https://img.shields.io/crates/v/rusty-patio.svg)](https://crates.io/crates/rusty-patio)

This repository contains core functionality to allow users to create plugins for the [Elgato Stream Deck](https://www.elgato.com/en/welcome-to-stream-deck), in [Rust](https://www.rust-lang.org/).

## How to Use

Install via cargo:

`cargo add rusty-patio`

Next, you need to get the arguments sent via the Stream Deck when it runs your plugin binary, connect to it, and then handle the events received.

```rust
// Create a new instance of the arguments that are received by the Stream Deck
let args = StreamDeckArgs::new();

// Connect to the Stream Deck
let client = connect_streamdeck(&args).await;

// Handle the Events
match client {
	Ok(mut client) => {
		while let Some(event) = client.received_events.recv().await {
			match event {
				EventReceived::WillAppear(event) => {
					let _ = client.log_message(format!("WillAppear Fired: {:?}", event)).await;
				}
				// the rest of the events you want to handle
			}
		}
	}
}
```

## Example

The exmaple directory in this repo contains a ready-to-run example counter plugin. All you have to do is run the build script for your platform, then drop that generated `.sdPlugin` directory into your Stream Deck Plugins directory.

### Windows

1. Run the `build.ps1` script
2. Copy the directory `target/com.example.rp-example.sdPlugin` to `%AppData%/Elgato/StreamDeck/Plugins`
3. Restart your Stream Deck software, and add the "Basic Counter" under "Custom" to a button, and press the button!

### macOS
1. Run the `build.sh` script
2. Copy the directory `target/com.example.rp-example.sdPlugin` to `~/Library/Application Support/com.elgato.StreamDeck/Plugins`
3. Restart your Stream Deck software, and add the "Basic Counter" under "Custom" to a button, and press the button!

⚠ **Note:** This repository is in its early stages, and as such there may be bugs. If you find any, please open an issue or PR!