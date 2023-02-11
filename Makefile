.PHONY: format build test clippy test-ci kafka

format:
	cargo fmt
	cargo tomlfmt 2> /dev/null

f: format

build:
	cargo build

b: build

kafka:
	docker compose up -d
	sleep 10

test-ci: kafka
	make test

test:
	cargo test --all-features
	cargo test --all-features -- --ignored

t: test

clippy:
	cargo clippy --all-features

c: clippy
