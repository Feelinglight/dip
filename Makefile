
run:
	cargo run

format:
	cargo format

web_setup:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk

web_build:
	cargo build --target wasm32-unknown-unknown

web_dev: web_build
	trunk serve ./crates/visualizer/src/web/index.html --open

web_release: web_build
	# Сборка в папку dist
	trunk build ./crates/visualizer/src/web/index.html --release
