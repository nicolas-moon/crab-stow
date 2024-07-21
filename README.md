# Crab-Stow (WIP)
Crab-Stow is a command line utility written in Rust that mirros the functionality of GNU stow.
It helps in managing the installation of sofware packages by creating and remvoving symbolic links to the 
target direcotries, allowing for a clean and maintainable structure.

## Goals

- Feature Parity with GNW Stow: Implement all core functionalities of GNU Stow to handle
symbolic links and manage the multiple package direcoties.
- Performance: Leverage Rust's performance benefits to create a fast and efficient CLI tool.
- Safety: Utilize Rust's safety garuntees to reduce runtime errors and improve reliabiltiy.
- Ease of Use: Provide a user-friendly command line interface with clear commands and arguments.
- Cross-Platform Compatibility: Ensure the tools works seamlessly on various operating systems, include Windows, macOS and Linux.

## Features
- Stow: Create symbolic links from the package directory to the target directory
- Unstow: Remove symbolic links from the target directory.
- recurisive Operations: Handle directories recurisvely to ensure all files and subdirectories are correctly linked or unlicked.


