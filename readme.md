# Note Manager

A simple and lightweight note manager with both CLI and web interfaces, built in Rust.

## Features

- **CRUD Operations**: Create, list, and delete notes.
- **Dual Interface**: Use it as a CLI tool or via a web API.
- **Data Persistence**: Stores your notes in a local SQLite database.
- **Clean Architecture**: Well-structured code with separation of concerns (db, service, handlers, cli).

## Technology Stack

- **Rust**: The core programming language.
- **SQLx**: For async, safe database operations.
- **Axum**: A modern web framework for the REST API.
- **Clap**: For powerful CLI argument parsing.
- **Serde**: For JSON serialization.
- **Chrono**: For handling date and time.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)


### Usage

The application can be run in two modes: **CLI** (default) and **Web**.

### CLI Mode

Manage your notes directly from the terminal.

### Web Mode

Manage your notes via get/post/put/delete queries



