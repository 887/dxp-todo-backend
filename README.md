# Example hello world app using Rust Poem Framework with Hot Lib Reload Functionality

Uses 3 tokio channels to communicate between tokio tasks.
Uses a mutex to synchronize reloads.

## Features

* **poem-web/poem**: built on top of the rust poem framework.
* **hot-lib-reloader-rs**: includes hot reload functionality.

### watch command lib only

`
cargo watch -w lib 'build -p lib'
`

### watch command lib and migrations

`
cargo watch -w lib -w migration -x 'build -p lib -p migration-runner'
`

### run

cargo run
