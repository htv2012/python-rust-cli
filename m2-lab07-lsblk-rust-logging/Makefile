DEV = sdb

.PHONY: info lint dev edit clean

info: lint
	cargo run info $(DEV)

lint:
	cargo check
	cargo clippy
	cargo fmt

dev: edit info

edit:
	nvim -p src/*.rs

clean:
	cargo clean
