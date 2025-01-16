.PHONY: \
	all \
	bench \
	build \
	cargo \
	clean \
	format \
	install \
	invoke \
	lint \
	watch \
	test \

### One dev cycle
all: invoke

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
watch: lint format build test
	cargo lambda watch

### Perform benchmark
bench:
	cargo lambda watch
	open target/criterion/report/index.html

### Delete generated files
clean:
	cargo clean

invoke:
	cargo lambda invoke --data-ascii '{"total": "200.0", "tip": "0.18"}'

