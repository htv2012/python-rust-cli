.PHONY: all clean format lint run

### Default target(s)
all: lint format run

### Perform static analysis
lint:
	cargo clippy

### Format code
format:
	cargo fmt

### Run the project
run:
	cargo run

### Remove generated files
clean:
	cargo clean
