.PHONY: \
	all \
	bench \
	build \
	cargo \
	clean \
	format \
	install \
	lint \
	run \
	send \
	test \

### One dev cycle
all: send

### Install required crates
install:
	#add derive feature to clap (cli tool)
	cargo add clap --features derive
	#add polars
	cargo add polars --features lazy

### Build release version
build:
	cargo build --release

### Static analysis
lint:
	cargo clippy

### Format code
format:
	cargo fmt

### Run all tests
test: lint format build
	cargo test

### Runsthe cli
run: test
	cargo lambda watch

### Perform benchmark
bench:
	cargo lambda watch
	open target/criterion/report/index.html

### Delete generated files
clean:
	cargo clean

send:
	cargo lambda invoke --data-ascii '{"total": "200.0", "tip": "0.18"}'

