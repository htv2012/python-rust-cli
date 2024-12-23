DEV = sda

.PHONY: info lint dev edit clean

info: lint
	cargo run --
	cargo run -- 10

lint:
	cargo check
	cargo clippy
	cargo fmt

dev: edit info

edit:
	nvim -p src/*.rs

clean:
	cargo clean
