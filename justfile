#.justfile

# Defautl recipe
default:
    @just --list

# Build the Rust Project
build:
  cargo build

# Run the project (probably)
run *ARGS:
  cargo run {{ARGS}}

# Install the CLI tool locally
install:
  cargo install --path .

# Uninstall the CLI Tool
uninstall:
  cargo uninstall crab-stow

# Run the tests
test:
  cargo test

# Format the code
fmt: 
  cargo fmt

# lint the code
lint: 
  cargo clippy

# Clean build artifacts
clean: 
  cargo clean

# Full development cycle: clean, build, test, lint
dev: clean build test lint

# Update Dependencies in the cargo.lock
update:
  cargo update

# Upgrade dependencies in the cargo.toml
upgrade:
  cargo upgrade
