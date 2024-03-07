cargo build --release
cargo run --bin uniffi-bindgen generate --library target/release/opaque_type.dll --no-format --language python --out-dir target/python