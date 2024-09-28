format:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

build:
	cargo build --release
