FROM rustlang/rust:nightly-bookworm-slim AS builder
RUN apt-get update -y && apt-get upgrade -y
RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/

RUN USER=root cargo new aleph-zero-login
WORKDIR /usr/src/aleph-zero-login
RUN touch src/lib.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
RUN rm src/*.rs
COPY assets ./assets
COPY src ./src
COPY static ./static
COPY azero-testnet-metadata.scale .
COPY index.html .
COPY rust-toolchain.toml .
RUN touch src/main.rs
RUN touch src/lib.rs
RUN cargo build --features=ssr --release --bin server
RUN apt-get install -y clang gcc-multilib build-essential
#RUN cargo update
RUN trunk build --features=hydration --release --public-url /dist/

FROM rustlang/rust:nightly-bookworm-slim

COPY --from=builder /usr/src/aleph-zero-login/target/release/server /bin/aleph-zero-login
COPY --from=builder /usr/src/aleph-zero-login/dist/ /dist/
USER 1000
ENTRYPOINT [ "aleph-zero-login" ]