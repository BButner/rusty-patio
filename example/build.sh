# Build the Solution
cargo build --release

# Make the plugin directory in the target directory
mkdir -p target/com.example.example.sdPlugin

# Copy the plugin manifest
cp manifest.json target/com.example.example.sdPlugin

# Copy the plugin binary
cp target/release/rp-example target/com.example.example.sdPlugin

# Copy the Images directory into the plugin directory
cp -r Images target/com.example.example.sdPlugin