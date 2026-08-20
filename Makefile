# Makefile for the Mirai Trade valuation engine.
#
# This is a library crate (no binary of its own) - it's consumed by the
# live trading engine (ssx-node) and API layer (ssx-executor) in the
# private application repo, neither of which live here. There's
# deliberately no `run`/`seed` target: there's nothing standalone to run.

.PHONY: all build test visualize clean

all: build

build:
	cargo build

test:
	cargo test

visualize:
	python3 visualize.py

clean:
	cargo clean
