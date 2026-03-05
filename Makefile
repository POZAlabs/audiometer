setup:
	uv sync
	uv run maturin develop --uv

publish:
	uv run maturin publish
