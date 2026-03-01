# Makefile for SSX Valuation Engine

.PHONY: all build run test seed clean

all: build

build:
	cargo build

run:
	cargo run -p ssx-node

test:
	cargo test

seed:
	cargo run -p ssx-node

visualize:
	python3 visualize.py

clean:
	cargo clean
