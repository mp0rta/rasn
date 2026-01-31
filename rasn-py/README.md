# rasn-py

Python bindings for rasn via PyO3.

## setup
```
uv sync
```
## build
```
uv run maturin develop
```

```
#./rusn-py
# clean
rm -rf ../target dist

# wheel build
uv run maturin build --release -o dist

# force reinstall into the active venv
uv run python -m pip install --force-reinstall dist/*.whl

# tests
uv run pytest -q
```
