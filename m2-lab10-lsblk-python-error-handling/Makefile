.PHONY: all test run lint

DEV = sdb

### Default target(s)
all: run

### Perform static analysis
lint:
	uv run ruff check --select I --fix .
	uv run ruff format .
	uv run ruff check . --fix

### Run the project
run: lint
	uv run blkpy info $(DEV)
	LOGLEVEL=debug uv run blkpy info $(DEV)


### Run unit tests
test: lint
	uv run pytest -s -v

clean:
	uv clean
	rm -fr .ruff_cache .venv
