DEV = sdb

.PHONY: all info lint dev edit clean error

all: error

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

### Error testing
error:
	RUST_BACKTRACE=1 PATH=./support:$(PATH) cargo run -- --log-to-file info $(DEV)

