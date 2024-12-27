DEV = sda

info: lint
	cargo run info $(DEV)

run: lint
	cargo run 

lint:
	cargo check
	cargo clippy
	cargo fmt

