# Example hello world app using Rust Axum with Hot Lib Reload Functionality

Uses tokio channels to communicate between tokio tasks.
Uses a mutex to synchronize reloads.

## Features

* **axum**: web framework for building async web services.
* **hot-lib-reloader-rs**: includes hot reload functionality.

### watch command lib only

`
cargo watch -w server 'build -p server'
`

### watch command lib and migrations

`
cargo watch -w server -w migration -x 'build -p server -p migration-runner'
`

### run

cargo run
