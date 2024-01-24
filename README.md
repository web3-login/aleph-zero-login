# aleph-zero-login
OpenId Connect server for Azero ID.

This is an experimental project to explore the possibilities of the [Azero ID](https://azero.id/) protocol. Do not use this in production.

It is an adoption of my [NFT Login](https://github.com/nft-login/nft-login) project to the [Azero ID](https://azero.id/) NFT.

## Motivation

Non fungible tokens are a proof for a digital ownership. This ownership can be used to give access to any digital resource or service.

## Features

- [x] Azero ID
- [x] OpenId Connect
- [ ] OAuth2
- [ ] SSO
- [ ] SAML

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [Axum](https://crates.io/crates/axum)
- [Subxt](https://crates.io/crates/subxt)
- [Yew](https://yew.rs/)
- [Trunk](https://trunkrs.dev/)
- [AZERO.ID](https://azero.id/)

## Quick Start

```sh
cargo build --features=ssr --release --bin server
trunk build --features=hydration --release --public-url /dist/
cargo run --features=ssr --bin server
```

## Test

```sh
cargo test --features=ssr -- --nocapture
```

## Configuration

### Contracts

```sh
ink-wrapper -m assets/azero_router_metadata.json | rustfmt --edition 2021 > src/azero/router_contract.rs
ink-wrapper -m assets/tzero_router_metadata.json | rustfmt --edition 2021 > src/tzero/router_contract.rs
```

### Polkadot Node Metadata

```sh
cargo install subxt-cli
subxt metadata --output-file azero-testnet-metadata.scale --url wss://ws.test.azero.dev:443
```

### Generate Keys

```sh
openssl genpkey -algorithm ed25519 -out assets/do_not_use.pem
```


### Build frontend

```sh
trunk build --features=hydration --release --public-url /dist/
```

### Develop frontend

```sh
trunk serve --features=hydration
```