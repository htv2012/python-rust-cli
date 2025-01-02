.PHONY: all test run lint clean

### Default target(s)
all: run

### Perform static analysis
lint:
	uv run ruff check --select I --fix ./python/tests
	uv run ruff format ./python/tests
	uv run ruff check ./python/tests --fix

### Run the project
run: lint
	uv run cow

### Run unit tests
test: build lint
	uv run pytest -s -v

### Clean up generated files
clean:
	cargo clean
	uv clean
	rm -fr .ruff_cache .venv



### Build the Rust library
build:
	uv run maturin build