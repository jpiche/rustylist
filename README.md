# RustyList

A stupidly simple Task List API written in Rust.

### Setup

1. Set environment variables. For local development, the app will look for `.env` in the root project directory. Any missing environment variables will cause the app to panic at initial run. A sample `.env` file can be found at `.env.sample`.
1. Create DB and run migrations. This can be done with `diesel setup`. By default diesel looks for the `DATABASE_URL` environment variable, but all options can be overridden via command line arguments. The diesel binary can be installed by running `cargo install diesel_cli --no-default-features --features postgres` (since only postgres support is required for this project).

### Running

The app can be ran by running `cargo run` in the project directory.
