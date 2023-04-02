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

### Run

```bash
cargo make run
```

## Docker
The easiest way to run the project is to use Docker.

First, build the image:
```
docker build -t zk-wordle .
```

Then, run it:
```
docker run -d -p 8080:8080 zk-wordle
```

You can now access the app at http://localhost:8080
