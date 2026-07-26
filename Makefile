
run:
	cargo run

format:
	cargo format

web_setup:
	rustup target add wasm32-unknown-unknown

web_build:
	cargo build --target wasm32-unknown-unknown
