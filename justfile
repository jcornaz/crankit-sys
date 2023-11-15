set dotenv-load

# Perform all verifications (compile, test, lint, etc.)
verify: test lint doc check-msrv
	cargo deny check licenses

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch --delay 0.1 --clear --why -- just verify

# Run the 'miminal' example
run-minimal:
    cargo +nightly playdate run --example minimal

# Run the 'hello world' example
run-hello-world:
    cargo +nightly playdate run --example hello_world

# Run the tests
test:
	cargo hack check --feature-powerset
	cargo hack test --tests --feature-powerset
	cargo test --doc --all-features

# Run the static code analysis
lint:
	cargo fmt -- --check
	cargo hack clippy --each-feature --all-targets

# Build the documentation
doc *args:
	cargo doc --all-features --no-deps {{args}}

# Open the documentation page
doc-open: (doc "--open")

# Make sure the MSRV is satisfiable
check-msrv:
	cargo msrv verify

# Clean up compilation output
clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
install-dev-tools:
	rustup install stable
	rustup override set stable
	cargo install cargo-hack cargo-watch cargo-msrv

# Install a git hook to run tests before every commits
install-git-hooks:
	echo '#!/usr/bin/env sh' > .git/hooks/pre-push
	echo 'just verify' >> .git/hooks/pre-push
	chmod +x .git/hooks/pre-push

release *args: verify
    test $GITHUB_TOKEN
    test $CARGO_REGISTRY_TOKEN
    cargo release {{args}}

generate-bindings-x86_64: (generate-bindings "x86_64")
generate-bindings-aarch64: (generate-bindings "aarch64")
generate-bindings-playdate: (generate-bindings "thumbv7em-none-eabihf")

generate-bindings target:
	bindgen wrapper.h \
		--use-core \
		--default-enum-style rust \
		--allowlist-type PlaydateAPI \
		--allowlist-type PDSystemEvent \
		--allowlist-type LCDSolidColor \
		--allowlist-type LCDColor \
		--allowlist-type LCDPattern \
		--allowlist-var LCD_COLUMNS \
		--allowlist-var LCD_ROWS \
		--bitfield-enum PDButtons \
		-- -DTARGET_EXTENSION -I$PLAYDATE_SDK_PATH/C_API -target {{target}}
