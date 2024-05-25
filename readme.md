# Example hello world app using Rust Poem Framework with Hot Lib Reload Functionality

Uses 3 tokio channels to communicate between tokio tasks.

## Features

* **Rust Poem Framework**: built on top of the Rust Poem framework.
* **Hot Lib Reload Functionality**: includes hot lib reload functionality, which allows you to make changes to your poem code and see the results immediately without having to restart your application.

### watch command

cargo watch -w lib -w migration -w migration-runner -x 'build -p lib'

### run

cargo run
