# Mind Up

The [laplace](https://github.com/noogen-projects/laplace)-application that upgrade the mind.

## Build

Building mindup requires the latest `stable` Rust toolchain, the `wasm32` target and `cargo-make` and
`wasm-bindgen` build tools.

To install Rust and its toolchain and target via [rustup](https://rustup.rs/), if it is not already installed, run:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

rustup toolchain install stable
rustup target add wasm32-unknown-unknown
```

To install [cargo-make](https://github.com/sagiegurari/cargo-make) and
[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), run:

```shell
cargo install --force cargo-make wasm-bindgen-cli
```

To build mindup and its example, run:

```shell
cargo make all
```

Or for a debug build, run:

```shell
cargo make all -p debug
```

## Run example

Run the laplace server with mindup application:

```shell
cargo make run
```

Or for a debug build:

```shell
cargo make run -p debug
```

Then visit [http://localhost:8080/mindup](http://localhost:8080/mindup). You can change the default port and other settings
by editing `app_runner/settings.toml` file.

## Development notes

To check the project, use the following command:

```shell script
cargo check --workspace --all-features --all-targets
```

To run all tests, use the following command:

```shell script
cargo test --all-features --all-targets
```

To check and perform formatting, use the following commands:

```shell script
cargo +nightly fmt -- --check
cargo +nightly fmt
```

To enable autoformatting for IntelliJ IDEA with the Rust plugin:

`File -> Settings -> Languages & Frameworks -> Rust -> Rustfmt, check "Run rustfmt on Save"`

To run clippy, use the following command:

```shell script
cargo clippy --all-targets --all-features -- -D warnings
```

To setup git hook, use the following command:

```shell script
cp .git-pre-push.sh .git/hooks/pre-push
```
