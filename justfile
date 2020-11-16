# usage
usage:
	@just -l

# prepare
prepare:
	@cargo fmt --help 2>&1 > /dev/null || rustup component add rustfmt
	@cargo clippy --help 2>&1 > /dev/null || rustup component add clippy

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
lint: prepare
	@cargo clippy

# test
test: prepare
	@cargo test

# clean
clean:
	@cargo clean

# vim: set noexpandtab :
