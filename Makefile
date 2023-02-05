.PHONY: format build test clippy

format:
	cargo fmt

f: format

build:
	cargo build

b: build

test:
	cargo build --all-features
	docker compose up -d &&\
		sleep 5 &&\
		cargo test --all-features -- --ignored && \
		docker compose down

t: test

clippy:
	cargo clippy --all-features

c: clippy
