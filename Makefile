build:
	cargo build --manifest-path=rust/Cargo.toml

build-arm:
	./rust/scripts/compile_armv6.sh

test:
	cargo test --manifest-path=rust/Cargo.toml
