# Makefile for SSX Valuation Engine

.PHONY: all build run test seed clean

all: build

build:
	cargo build

run:
	cargo run

test:
	cargo test

seed:
	cargo run

clean:
	cargo clean
