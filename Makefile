.PHONY: format build test clippy test-ci

format:
	cargo fmt

f: format

build:
	cargo build

b: build

test-ci:
	cargo build --all-features
	docker compose up -d &&\
		sleep 3 &&\
		make test

test:
	cargo test  --all-features
	cargo test --all --all-features -- --ignored

t: test

clippy:
	cargo clippy --all-features

c: clippy
