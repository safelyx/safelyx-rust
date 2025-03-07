SHELL := /bin/bash

.PHONY: install
install:
	rustup install 1.85
	rustup component add rustfmt

.PHONY: format
format:
	rustfmt +nightly --edition 2024 src/*.rs tests/*.rs

.PHONY: test
test:
	rustfmt +nightly --edition 2024 --check src/*.rs tests/*.rs
	cargo test

.PHONY: publish
publish:
	cargo clean
	cargo build --release
	cargo publish
