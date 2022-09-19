.PHONY: format build test clippy

format:
	cargo fmt

f: format

build:
	cargo build

b: build

test:
	cargo build --all-features
	cargo test --all-features
	cargo test --all-features -- --ignored

t: test

clippy:
	cargo clippy --all-features

c: clippy
