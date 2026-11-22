#!/bin/sh
# Regenerate python/slai/_slai.pyi from Rust via pyo3 experimental-inspect.
# Run after any FFI change, then: python -m mypy.stubtest slai, on a fresh build
set -e
cd "$(dirname "$0")/.."
out=$(mktemp -d)
trap 'rm -rf "$out"' EXIT
uvx maturin@1.16 generate-stubs --out "$out"
# Black puts each `__all__` name and each argument of a long signature on its own line
uvx black@23.12.1 --quiet --line-length 99 "$out/slai/_slai.pyi"
cp "$out/slai/_slai.pyi" python/slai/_slai.pyi
echo "wrote python/slai/_slai.pyi"
