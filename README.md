# Example hello world app using Rust Axum with Hot Lib Reload Functionality

Uses tokio channels to communicate between tokio tasks.
Uses a mutex to synchronize reloads.

## Features

* **axum**: web framework for building async web services.
* **hot-lib-reloader-rs**: includes hot reload functionality.

### watch command lib only

`
cargo watch -w heart 'build -p heart'
`

### watch command lib and migrations

`
cargo watch -w heart -w migration -x 'build -p heart -p migration-runner'
`

### run

cargo run
