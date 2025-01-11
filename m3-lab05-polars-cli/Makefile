### One dev cycle
all: run

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
test:
	cargo test

### Runsthe cli
run: lint format build test
	cargo run --

### Perform benchmark
bench:
	cargo bench
	open target/criterion/report/index.html

### Delete generated files
clean:
	cargo clean

