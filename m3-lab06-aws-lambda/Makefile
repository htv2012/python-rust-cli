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

### Deploy to remote server
deploy:
	cargo lambda deploy

### Install required crates
install:
	#add derive feature to clap (cli tool)
	cargo add clap --features derive
	#add polars
	cargo add polars --features lazy

### Build release version
build:
	cargo lambda build --release --arm64

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
	cargo lambda watch:
	open target/criterion/report/index.html

### Delete generated files
clean:
	cargo clean

### Invoke the function locally
invoke:
	bash invoke.sh 200.0 0.18

### Invoke the function remotely
invoke-remote:
	cargo lambda invoke \
		--remote \
		--data-ascii '{"total": "200.0", "tip": "0.18"}' \
		hv-tip-calculator

### Static analysis
lint:
	cargo clippy

