# kettolab-todo-cli

[![CI](https://github.com/urotanketto/kettolab-todo-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/urotanketto/kettolab-todo-cli/actions/workflows/ci.yml)

A simple CLI Todo app written in Rust.

This repository is part of the **kettolab** series — an experimental playground for building simple, functional tools and applications through hands-on learning.

This project is intended to help improve development skills by implementing and iterating on small ideas, starting with a command-line todo manager.

---

## Features

- 📝 Add, update, mark done, and list tasks from the command line
- 🗃 Simple task storage using a local JSON file

---

## Development

This project uses GitHub Actions to enforce code quality on every push:

- ✅ `cargo fmt` – checks code formatting
- ✅ `cargo clippy` – checks for common mistakes and style issues
- ✅ `cargo test` – runs all unit tests to verify core functionality

You can find the CI workflow in [.github/workflows/ci.yml](.github/workflows/ci.yml)

To run checks locally:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

---

## Usage

Build and run the application with Cargo:

```bash
cargo build
cargo run -- [COMMAND] [ARGS]
```

### Commands
```
# Add a new task
cargo run -- add "Buy milk"

# List all tasks
cargo run -- list

# Mark a task as done
cargo run -- done 1

# Update a task's title
cargo run -- update 1 "Buy oat milk"
```
