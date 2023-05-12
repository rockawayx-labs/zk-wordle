# ZK WORDLE 

The app is deployed here: [http://zkwordle.rockawayx.com](http://zkwordle.rockawayx.com)

![screenshot-zk-wordle](https://user-images.githubusercontent.com/44506010/229332289-2d0a762e-380b-42ac-8fba-32690e533fd1.png)

## Project Structure

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
### Kill

If you kill the process by pressing `CMD+C`  (or `CTRC+C`) it kill only the frontend running on port 8080. 
Make sure to kill the server running on port 9000 too. 

List all processes running on port 9000:
```bash
lsof -i tcp:9000
```

Kill process given its PID
```bash
kill -9 <PID>
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


## System architecture
![architecture](https://github.com/RBFLabs/zk-wordle/assets/44506010/e9f53fe1-9874-4ee8-a147-1d9b8137e241)

## Game flow
![game-flow](https://github.com/RBFLabs/zk-wordle/assets/44506010/9ac8fb73-c96a-4f3e-9182-a46c60f92341)



