.PHONY: lint test build clean

build: lint test
	uv tool run maturin build --release

test: lint
	cargo test

lint:
	cargo check
	cargo clippy
	cargo fmt

clean:
	cargo clean