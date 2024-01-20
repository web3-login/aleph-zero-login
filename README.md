# aleph-zero-login
OpenId Connect server for Azero ID.

## Quick Start

```sh
cargo build
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
