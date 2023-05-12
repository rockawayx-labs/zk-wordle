FROM rust:1.67 AS builder

# Install the required dependencies
RUN apt-get update && apt-get install -y curl gnupg

# Install Node.js LTS
RUN curl -sL https://deb.nodesource.com/setup_lts.x | bash -
RUN apt-get update && apt-get install -y nodejs

# Install the latest Yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor | tee /usr/share/keyrings/yarn-keyring.gpg >/dev/null
RUN echo "deb [signed-by=/usr/share/keyrings/yarn-keyring.gpg] https://dl.yarnpkg.com/debian stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update && apt-get install -y yarn

RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-make
RUN cargo install wasm-pack

WORKDIR /usr/src/zk-wordle

COPY ./Cargo.* ./*.toml ./
COPY ./backend ./backend
COPY ./contract ./contract
COPY ./core ./core
COPY ./frontend ./frontend
COPY ./methods ./methods
COPY ./wasm-verifier ./wasm-verifier

RUN cargo make build

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y nginx-light ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/zk-wordle

COPY ./nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /usr/src/zk-wordle/target/release/backend ./target/release/backend
COPY --from=builder /usr/src/zk-wordle/frontend/dist /var/www/html

EXPOSE 8080

ENTRYPOINT nginx && /usr/src/zk-wordle/target/release/backend
