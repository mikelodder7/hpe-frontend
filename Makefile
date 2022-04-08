.PHONY: build test

.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: build
build:
	wasm-pack build
