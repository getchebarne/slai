#!/bin/sh
# Regenerate python/slai/slai/__init__.pyi from Rust via pyo3 experimental-inspect.
# Run after any FFI change, then: python tests/test_stub_conformance.py
set -e
cd "$(dirname "$0")/.."
uvx maturin@1.13 generate-stubs --out /tmp/slai-stubs
# pyo3 emits __eq__/__ne__ typed against the class; mypy requires `object`
sed -E 's/def (__eq__|__ne__)\(self, \/, other: [^)]+\)/def \1(self, other: object)/' \
    /tmp/slai-stubs/slai.pyi > python/slai/slai/__init__.pyi
echo "wrote python/slai/slai/__init__.pyi"
