# Crab-Stow (WIP)
A Rust reimplementation of GNU stow for managing symlink farms.

## Features
- Stow and unstow packages
- Simulate mode
- Verbose logging
- Cross-platform support


## Installation

### From source

```bash
git clone https://github.com/nicolas-moon/crab-stow.git
cd crab-stow
cargo build --release
```

### Justfile

```bash
just install
```

## Usage
```bash
# Stow a package
crab-stow dotfiles

# Unstow a package
crab-stow -D dotfiles

# Simulate a stow
crab-stow -n dotfiles
```

## Options
- `-t, --target`: Set target directory (default: current directory)
- `-d, --dir`: Set stow directory (default: current directory)
- `-n, --no-act`: Simulate changes without making them
- `-v, --verbose`: Increase verbosity (`-v` for info, `-v -v` for debug)
- `-R, --restow`: Unstow and then stow package
- `--adopt`: Adopt existing files into stow (WIP)
- `--no-folding`: Disable directory folding (WIP)

## Goals

- Feature Parity with GNU Stow: Implement all core functionalities of GNU Stow to handle
symbolic links and manage multiple package directories.
- Performance: Leverage Rust's performance benefits to create a fast and efficient CLI tool.
- Safety: Utilize Rust's safety guarantees to reduce runtime errors and improve reliability.
- Ease of Use: Provide a user-friendly command line interface with clear commands and arguments.
- Cross-Platform Compatibility: Ensure the tool works seamlessly on various operating systems, including Windows, macOS and Linux.

## License

MIT License
