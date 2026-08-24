# justfile

# Default recipe
default:
    @just --list

# Build the Rust project
build:
    cargo build

# Run the project, passing through any args (e.g. `just run -n dotfiles`)
run *ARGS:
    cargo run {{ARGS}}

# Install the CLI tool locally
install:
    cargo install --path .

# Uninstall the CLI tool
uninstall:
    cargo uninstall crab-stow

# Run the tests
test:
    cargo test

# Format the code
fmt:
    cargo fmt

# Check formatting without applying changes
fmt-check:
    cargo fmt --check

# Lint the code
lint:
    cargo clippy --all-targets

# Clean build artifacts
clean:
    cargo clean

# Full development cycle: clean, build, test, lint (runs sequentially)
dev:
    just clean
    just build
    just test
    just lint

# Update dependencies in the Cargo.lock
update:
    cargo update

# Upgrade dependencies in the Cargo.toml (requires the cargo-upgrade subcommand)
upgrade:
    cargo upgrade
