# Crab-Stow (WIP)
A Rust reimplementation of GNU stow for managing symlink farms.

## Features
- Stow and unstow packages
- simulate mode
- verbose logging
- Cross-platform support


## Installation
```bash
git clone https://github.com/nicolas-moon/crab_stow.git
cd crab_stow
cargo build --release
```

## Usage
```bash
# Stow a package
./crab_stow dotfiles

# Unstow a package
./crab_stow -D dotfiles

# Simulate a stow
./crab_stow -n dotfiles
```

## Configuration
- `-t, --target`: Set target directory (default: current directory)
- `-d, --dir`: Set stow directory (default: current directory)
- `-n, --no-act`: Simulate changes without making them
- `v, --verbose`: Increase verbosity
- `-R , --restow`: Unstow and then stow package
- `--adopt`: adopt existing files into stow
- `--no-folding`: Disable directory folding

## Goals

- Feature Parity with GNW Stow: Implement all core functionalities of GNU Stow to handle
symbolic links and manage the multiple package direcoties.
- Performance: Leverage Rust's performance benefits to create a fast and efficient CLI tool.
- Safety: Utilize Rust's safety garuntees to reduce runtime errors and improve reliabiltiy.
- Ease of Use: Provide a user-friendly command line interface with clear commands and arguments.
- Cross-Platform Compatibility: Ensure the tools works seamlessly on various operating systems, include Windows, macOS and Linux.

### License

MIT License
