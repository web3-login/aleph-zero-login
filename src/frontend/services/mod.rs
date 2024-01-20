//#![cfg(target_arch = "wasm32")]

use anyhow::anyhow;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use serde_json::json;
use subxt::ext::codec::Encode;
use subxt::tx::PartialExtrinsic;
use subxt::utils::AccountId32;
use subxt::{self, OnlineClient, PolkadotConfig};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[subxt::subxt(runtime_metadata_path = "azero-testnet-metadata.scale")]
pub mod polkadot {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getAccounts)]
    pub fn js_get_accounts() -> Promise;
    #[wasm_bindgen(js_name = signPayload)]
    pub fn js_sign_payload(payload: String, source: String, address: String) -> Promise;
}

/// DTO to communicate with JavaScript
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// account name
    pub name: String,
    /// name of the browser extension
    pub source: String,
    /// the signature type, e.g. "sr25519" or "ed25519"
    pub ty: String,
    /// ss58 formatted address as string. Can be converted into AccountId32 via it's FromStr implementation.
    pub address: String,
}

pub async fn get_accounts() -> Result<Vec<Account>, anyhow::Error> {
    let result = JsFuture::from(js_get_accounts())
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let accounts_str = result
        .as_string()
        .ok_or(anyhow!("Error converting JsValue into String"))?;
    let accounts: Vec<Account> = serde_json::from_str(&accounts_str)?;
    Ok(accounts)
}

fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    format!("0x{}", hex::encode(bytes.as_ref()))
}

fn encode_to_hex<E: Encode>(input: &E) -> String {
    format!("0x{}", hex::encode(input.encode()))
}

/// this is used because numeric types (e.g. u32) are encoded as little-endian via scale (e.g. 9430 -> d6240000)
/// while we need a big-endian representation for the json (e.g. 9430 -> 000024d6).
fn encode_to_hex_reverse<E: Encode>(input: &E) -> String {
    let mut bytes = input.encode();
    bytes.reverse();
    format!("0x{}", hex::encode(bytes))
}

/// communicates with JavaScript to obtain a signature for the `partial_extrinsic` via a browser extension (e.g. polkadot-js or Talisman)
///
/// Some parameters are hard-coded here and not taken from the partial_extrinsic itself (mortality_checkpoint, era, tip).
pub async fn extension_signature_for_partial_extrinsic(
    partial_extrinsic: &PartialExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
    api: &OnlineClient<PolkadotConfig>,
    account_id: &AccountId32,
    account_source: String,
    account_address: String,
) -> Result<Vec<u8>, anyhow::Error> {
    let spec_version = encode_to_hex_reverse(&api.runtime_version().spec_version);
    let transaction_version = encode_to_hex_reverse(&api.runtime_version().transaction_version);
    let mortality_checkpoint = encode_to_hex(&api.genesis_hash());
    let era = encode_to_hex(&subxt::config::extrinsic_params::Era::Immortal);
    let genesis_hash = encode_to_hex(&api.genesis_hash());
    let method = to_hex(partial_extrinsic.call_data());
    let nonce = api.tx().account_nonce(account_id).await?;
    let nonce = encode_to_hex_reverse(&nonce);
    let signed_extensions: Vec<String> = api
        .metadata()
        .extrinsic()
        .signed_extensions()
        .iter()
        .map(|e| e.identifier().to_string())
        .collect();
    let tip = encode_to_hex(&subxt::config::polkadot::PlainTip::new(0));

    let payload = json!({
        "specVersion": spec_version,
        "transactionVersion": transaction_version,
        "address": account_address,
        "blockHash": mortality_checkpoint,
        "blockNumber": "0x00000000",
        "era": era,
        "genesisHash": genesis_hash,
        "method": method,
        "nonce": nonce,
        "signedExtensions": signed_extensions,
        "tip": tip,
        "version": 4,
    });

    let payload = payload.to_string();
    let result = JsFuture::from(js_sign_payload(payload, account_source, account_address))
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let signature = result
        .as_string()
        .ok_or(anyhow!("Error converting JsValue into String"))?;
    let signature = hex::decode(&signature[2..])?;
    Ok(signature)
}
