# Rust Axum Zero to Production

This is my attempt to follow along the zero2prod book and implementing it with the Axum framework instead of the Actix framework

## Linker

For this project mold is used as the linker, see repo for guide.

## Inner development loop

### cargo-watch

#### install

`cargo install cargo-watch`

#### run

`cargo watch -x check -x test -x run`

## Continuous Integration

### Code coverage

#### install

`cargo install cargo-tarpaulin`

#### run

`cargo tarpaulin --ignore-tests`

### Linting

clippy should be installed by default by rustup

#### install

`rustup component add clippy`

#### run

`cargo clippy`

#### run ci

`cargo clippy -- -D warnings`

#### mute clippy

add attribute to the affected code:

`#[allow(clippy::lint_name)]`

### Formatting

rustfmt should be installed by default by rustup

#### install

`rustup component add rustfmt`

#### run

`cargo fmt`

#### run ci

`cargo fmt -- --check`

### Security Vulnerabilities

#### install

`cargo install cargo-audit`

#### run

`cargo audit`

## Debugging tools

### Cargo watch

#### install

`cargo install cargo-expand`

cargo-expand needs the nightly rust toolchain to work:
`rustup toolchain install nightly --allow-downgrade`

in order to use the nightly toolchain ONLY for the cargo-expand command use:
`cargo +nightly expand`

