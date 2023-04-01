# ZKHack Lisbon RockawayX

## Structure

This is a multi-package workspace (see definition in [`Cargo.toml`](Cargo.toml)).

- [frontend](client/): react app using wasm proof verifier
- [backend](backend/): backend using [Actix](https://github.com/actix/actix-web)
- [core](core/): shared core library
- [methods](methods/): source code from which ELF and ID is generated for Provers

## Requirements

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.

```bash
rustup target add wasm32-unknown-unknown
```

[`wasm-pack`](https://rustwasm.github.io/wasm-pack/) for bundling verifier function to WASM

```bash
cargo install wasm-pack
```

[`cargo-make`](https://github.com/sagiegurari/cargo-make) to run tasks defined in `Makefile.toml`.

```bash
cargo install cargo-make
```

## Tasks

See the [`Makefile.toml`](Makefile.toml) for list of all available tasks.

### Build

```bash
cargo make build
```

You can also use `cargo make build-release` for release version.

### Run

```bash
cargo make run
```

You can also use `cargo make run-release` to run the release version.

### Clean

```bash
cargo make clean
```
