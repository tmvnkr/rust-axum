## Linker

For this project mold is used as the linker

## Inner development loop

### cargo-watch

#### install

`cargo install cargo-watch`

#### run

`cargo watch -x chekc -x test -x run`

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
