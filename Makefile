.PHONY: run check test

run:
	solverforge server

check:
	solverforge check
	cargo check

test:
	solverforge test
	uv run pytest
