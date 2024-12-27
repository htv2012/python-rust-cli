DEV = sda

.PHONY: info lint dev edit clean

info: lint
	cargo run -q --
	cargo run -q -- -f "%H:%M:%S" 10

lint:
	cargo check
	cargo clippy
	cargo fmt

dev: edit info

edit:
	nvim -p src/*.rs

clean:
	cargo clean
