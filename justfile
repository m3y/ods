# usage
usage:
	@just -l

# prepare
prepare:
	@cargo watch --help 2>&1 > /dev/null || cargo install cargo-watch
	@cargo audit --help 2>&1 > /dev/null || cargo install cargo-audit

# run
run +TARGET:
	@cargo run --bin "{{TARGET}}"

# build
build: prepare
	@RUSTC_WRAPPER={{`which sccache`}} cargo build

# fmt
fmt: prepare
	@cargo fmt

# lint
lint:
	@cargo clippy

# audit
audit: lint
	@cargo audit

# test
test: prepare
	@cargo test

# clean
clean:
	@cargo clean

# vim: set noexpandtab :
