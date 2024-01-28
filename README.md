# 🌟 aleph-zero-login

[![Online](https://img.shields.io/badge/online-azero.id-blue)](https://azero.web3-login.net/)
[![Build Status](https://github.com/web3-login/aleph-zero-login/actions/workflows/rust.yml/badge.svg)](https://github.com/web3-login/aleph-zero-login/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

🔐 OpenId Connect server for Azero ID.

🚀 This is an experimental project to explore the possibilities of the [Azero ID](https://azero.id/) protocol. Do not use this in production.

🛠 It is an adoption of my [NFT Login](https://github.com/nft-login/nft-login) project to the [Azero ID](https://azero.id/) NFT.

🌐 The server is online at [azero.web3-login.net](https://azero.web3-login.net/).

## 💡 Motivation

💎 Non fungible tokens are a proof of digital ownership. This ownership can be used to give access to any digital resource or service.

## 📚 How it works

🔐 The server is an OpenId Connect server. It uses the [Azero ID](https://azero.id/) NFT as a proof of ownership. The user can login with the [Azero ID](https://azero.id/) NFT. The server verifies the signature of the token and returns a JWT token. The JWT token can be used to authenticate the user.

🔑 The server can be configured to use RSA or EDDSA keys. The keys are used to sign the JWT token. The public keys can be viewed at the /jwk endpoint.

📝 The server can be configured with a config.yml file. The config.yml file can be used to configure the keys and the OpenId Connect endpoints.

![Flow](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/web3-login/aleph-zero-login/main/flow.puml)

## 🌈 Features

- ✅ Azero ID
- ✅ OpenId Connect
- 🔜 OAuth2

## 🏗 Build on top of

- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- 🌐 [Axum](https://crates.io/crates/axum)
- 🔄 [Subxt](https://crates.io/crates/subxt)
- 🖼 [Yew](https://yew.rs/)
- 📦 [Trunk](https://trunkrs.dev/)
- 🔗 [AZERO.ID](https://azero.id/)

## 🚀 Quick Start

```sh
trunk build
cargo run --bin server
```

## 🧪 Test

```sh
cargo test -- --nocapture
```

## ⚙️ Configuration

### 📜 Contracts

🔧 The contracts are generated with ink-wrapper from the [Metadata](https://docs.azero.id/developers/deployments).

```sh
ink-wrapper -m assets/azero_router_metadata.json | rustfmt --edition 2021 > src/azero/router_contract.rs
ink-wrapper -m assets/tzero_router_metadata.json | rustfmt --edition 2021 > src/tzero/router_contract.rs
```

### 🌐 Polkadot Node Metadata

🔍 The metadata can be fetched from the node.

```sh
cargo install subxt-cli
subxt metadata --output-file azero-testnet-metadata.scale --url wss://ws.test.azero.dev:443
```

### 🔑 Generate Keys

🔐 We can generate keys with openssl. They are used to sign the tokens. The generated public keys can be viewed at /jwk endpoint.

```sh
openssl genpkey -algorithm ed25519 -out private_eddsa.pem
openssl genrsa --traditional -out private_rsa.pem 1024
```

📝 Copy the content into the config.yml as `rsa_pem` or `eddsa_pem` or add the path to the file as `rsa_pem_file`.

### 🌍 Build frontend

```sh
trunk build
```

### 🏗 Build backend

```sh
cargo build
```

### 🖥 Develop frontend

```sh
trunk serve
```

### 🚀 Run backend

```sh
cargo run --bin server
```

### 🐳 Run backend with docker

```sh
docker-compose up
```

## 📋 TODO

- [ ] Add tests
- [ ] Add documentation
- [ ] Add OAuth2
- [ ] ⚠️ Critical: The signature relies on the nonce, the user brings with the request. This is not secure. The id to signature should be generated by the server.

## 📜 License

[MIT](LICENSE)

## ⚠️ Warning

🔒 This is experimental software. Use at your own risk. One security risk is, that the server does not verify the client id and client secret of a server that wants to authenticate a user. This means that any server can fetch information of a user token if the user has logged in to the server before. The token is a uuid v4 and can hardly be guessed. Just saying.

## 🚫 Disclaimer

🤖 This is not an official Azero ID project. I am not affiliated with the Azero ID team. Use at your own risk.
