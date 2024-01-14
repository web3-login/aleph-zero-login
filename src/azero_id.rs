use std::str::FromStr;
use web3_login::nft_owner::NFTOwner;
use web3_login::signature_validator::SignatureValidator;

use hex::FromHex;
use sp_core::sr25519::{Public as Sr25519Public, Signature as Sr25519Signature};
use sp_core::{crypto::AccountId32, Pair};

use crate::azero::get_owner as get_azero_owner;
use crate::chain::Chain;
use crate::tzero::get_owner as get_tzero_owner;

#[derive(Default)]
pub struct AzeroId {}

impl AzeroId {
    pub fn new() -> Self {
        AzeroId {}
    }
}

impl SignatureValidator for AzeroId {
    fn validate_signature(&self, account: String, nonce: String, signature: String) -> bool {
        let account_id = match AccountId32::from_str(&account) {
            Ok(acc) => acc,
            Err(_) => return false,
        };

        let public_key = Sr25519Public::from_raw(account_id.into());

        let signature_bytes = match Vec::from_hex(signature.trim_start_matches("0x")) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        let signature = match Sr25519Signature::from_slice(&signature_bytes) {
            Some(sig) => sig,
            None => return false,
        };
        sp_core::sr25519::Pair::verify(&signature, nonce.as_bytes(), &public_key)
    }
}

impl NFTOwner for AzeroId {
    fn is_nft_owner(
        &self,
        _contract: String,
        account: String,
        nft: Option<String>,
        chain: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let chain = Chain::from_str(&chain)?;
        let rt = tokio::runtime::Runtime::new()?;

        match nft {
            Some(nft) => return Ok(rt.block_on(Self::is_nft_owner_of(&chain, account, nft))?),
            None => Ok(false),
        }
    }
}

impl AzeroId {
    pub async fn is_nft_owner_of(
        chain: &Chain,
        account: String,
        domain: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match chain {
            Chain::Azero => match get_azero_owner(domain).await {
                Ok(owner) => Ok(owner == account),
                Err(e) => Err(e),
            },
            Chain::AzeroTest => match get_tzero_owner(domain).await {
                Ok(owner) => Ok(owner == account),
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_nft_owner() {
        let account: String = "5Esx8QLfERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY1fn".to_string();
        let domain: String = "chriamue.azero".to_string();

        let is_owner = AzeroId::is_nft_owner_of(&Chain::Azero, account, domain)
            .await
            .unwrap();
        assert_eq!(is_owner, true);
    }

    #[tokio::test]
    async fn test_is_not_nft_owner() {
        let account: String = "5Esx8000ERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY000".to_string();
        let domain: String = "chriamue.azero".to_string();

        let is_owner = AzeroId::is_nft_owner_of(&Chain::Azero, account, domain)
            .await
            .unwrap();
        assert_eq!(is_owner, false);
    }

    #[tokio::test]
    async fn test_wront_tld() {
        let account: String = "5Esx8QLfERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY1fn".to_string();
        let domain: String = "chriamue.tzero".to_string();

        assert!(AzeroId::is_nft_owner_of(&Chain::Azero, account, domain)
            .await
            .is_err());
    }

    #[test]
    fn test_validate_signature() {
        let nonce = "This is a text message";
        let signature = "0x2aeaa98e26062cf65161c68c5cb7aa31ca050cb5bdd07abc80a475d2a2eebc7b7a9c9546fbdff971b29419ddd9982bf4148c81a49df550154e1674a6b58bac84";
        let account = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

        let azero_id = AzeroId::new();
        assert_eq!(
            azero_id.validate_signature(
                account.to_string(),
                nonce.to_string(),
                signature.to_string()
            ),
            true
        );
    }
}
