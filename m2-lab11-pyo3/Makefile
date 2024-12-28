.PHONY: all help run clean

all: run

help:
	@echo to run: 

run:
	uv run src/fact.py

clean:
	cargo clean
	uv clean
	rm -fr .venv .ruff_cache
