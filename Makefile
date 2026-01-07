.PHONY: lint build update

lint:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo check --release --locked

build:
	cargo build --release
	ls -lh target/release/macnetmon

update:
	# cargo install cargo-edit (if not installed)
	cargo upgrade -i
