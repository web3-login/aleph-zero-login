# aleph-zero-login
OpenId Connect server for Azero ID.

## Quick Start

```sh
cargo build
cargo run
```

## Configuration

### Polkadot Node Metadata

```sh
cargo install subxt-cli
subxt metadata --output-file azero-testnet-metadata.scale --url wss://ws.test.azero.dev:443
```
