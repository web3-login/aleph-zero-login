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
trunk build
cargo run --bin server
```

## Test

```sh
cargo test -- --nocapture
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
openssl genpkey -algorithm ed25519 -out private_eddsa.pem
openssl genrsa --traditional -out private_rsa.pem 1024
```


### Build frontend

```sh
trunk build
```

### Develop frontend

```sh
trunk serve
```

## License

[MIT](LICENSE)

## Warning

This is experimental software. Use at your own risk.
One security risk is, that the server does not verify the client id and client secret of a server that wants to authenticate a user.
This means that any server can fetch information of a user token if the user has logged in to the server before.
The token is a uuid v4 and can hardly be guessed. Just saying.

## Disclaimer

This is not an official Azero ID project. I am not affiliated with the Azero ID team. Use at your own risk.
