FROM rustlang/rust:nightly-bookworm-slim AS builder
RUN apt-get update -y && apt-get upgrade -y
RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
RUN apt-get install -y clang gcc-multilib build-essential libssl-dev pkg-config
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/

RUN USER=root cargo new aleph-zero-login
WORKDIR /usr/src/aleph-zero-login
COPY Cargo.toml Cargo.lock ./
RUN rm src/*.rs
COPY backend ./backend
COPY frontend ./frontend
COPY Trunk.toml .
COPY rust-toolchain.toml .
RUN cargo build --release --bin server
RUN trunk build

FROM rustlang/rust:nightly-bookworm-slim

COPY --from=builder /usr/src/aleph-zero-login/target/release/server /bin/aleph-zero-login
COPY --from=builder /usr/src/aleph-zero-login/dist/ /dist/
USER 1000
ENTRYPOINT [ "aleph-zero-login" ]