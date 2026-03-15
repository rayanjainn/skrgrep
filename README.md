# skrgrep

A high-performance, multi-threaded line-oriented search tool implemented in Rust.

## Overview

skrgrep is designed for speed and efficiency, leveraging Rust's concurrency primitives to provide faster-than-sequential searching. It features recursive directory traversal with automatic exclusion of common metadata and build directories.

## Features

- **Multi-threaded execution**: Utilizes a work-stealing thread pool for parallel file processing.
- **Fast Traversal**: Efficiently walks directory trees while skipping `.git`, `node_modules`, and `target` directories by default.
- **Color Highlighting**: Automatically highlights query matches in the terminal using ANSI escape codes.
- **Case Sensitivity**: Built-in support for both case-sensitive and case-insensitive matching.
- **Modern CLI**: Comprehensive help and version information provided via standard flags.

## Installation

### Homebrew

```bash
brew install skrgrep
```

### From Source

Ensure you have the Rust toolchain installed.

```bash
git clone https://github.com/rayanjainn/minigrep.git
cd minigrep
cargo install --path .
```

## Usage

Basic search syntax:

```bash
skrgrep <QUERY> [PATH]
```

If `PATH` is omitted, the current directory is used.

### Options

- `-i, --ignore-case`: Perform a case-insensitive search.
- `-h, --help`: Display help information.
- `-V, --version`: Display version information.

### Examples

Search for a string in the current directory:
```bash
skrgrep "main"
```

Search for a string in a specific directory case-insensitively:
```bash
skrgrep "TODO" ./src --ignore-case
```

## License

MIT
