.PHONY: release-build debug-build musl-build format
release-build:
	cargo build --release
debug-build:
	cargo build
musl-build:
	cargo build --release --target=x86_64-unknown-linux-musl
format:
	cargo format
	