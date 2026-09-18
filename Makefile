.PHONY: ingest run check test

ingest:
	uv run python tools/jev_ingest.py --tasks input/projects/platform-reliability/tasks.json --output generated/projects/platform-reliability/jev_candidates.json

run:
	solverforge server

check:
	solverforge check
	cargo check

test:
	solverforge test
	uv run pytest
