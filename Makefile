ARM64_BINARY := target/aarch64-apple-darwin/release/macnetmon
X86_64_BINARY := target/x86_64-apple-darwin/release/macnetmon
UNIVERSAL_BINARY := target/universal-apple-darwin/release/macnetmon

.PHONY: lint build build-universal update

lint:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo check --release --locked

build:
	cargo build --release
	ls -lh target/release/macnetmon

build-universal:
	cargo build --release --locked --target aarch64-apple-darwin
	cargo build --release --locked --target x86_64-apple-darwin
	mkdir -p $(dir $(UNIVERSAL_BINARY))
	lipo -create $(ARM64_BINARY) $(X86_64_BINARY) -output $(UNIVERSAL_BINARY)
	codesign --force --sign - $(UNIVERSAL_BINARY)
	lipo $(UNIVERSAL_BINARY) -verify_arch arm64 x86_64
	codesign --verify --all-architectures --verbose=2 $(UNIVERSAL_BINARY)
	$(UNIVERSAL_BINARY) --version

update:
	# cargo install cargo-edit (if not installed)
	cargo upgrade -i
