
run:
	cargo run -p dip

format:
	cargo fmt --all

web_setup:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk

web_build:
	cargo build -p dip --target wasm32-unknown-unknown

web_dev: web_build
	cd crates/visualizer && trunk serve src/web/index.html --open

web_release: web_build
	# Сборка в папку dist
	cd crates/visualizer && trunk build --release src/web/index.html
