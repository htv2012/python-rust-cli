.PHONY: all test run lint
SCRIPT := simple.py

### Default target(s)
all: run

### Perform static analysis
lint:
	ruff check --select I --fix .
	ruff format .
	ruff check . --fix

### Run the project
run: lint
	python3 $(SCRIPT) lvmlmde

### Run unit tests
test: lint
	pytest -s -v

