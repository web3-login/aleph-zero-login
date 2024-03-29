use std::str::FromStr;
use web3_login::signature_validator::SignatureValidator;

use hex::FromHex;
use sp_core::sr25519::{Public as Sr25519Public, Signature as Sr25519Signature};
use sp_core::{crypto::AccountId32, Pair};

use super::azero::get_owner as get_azero_owner;
use super::tzero::get_owner as get_tzero_owner;
use crate::chain::Chain;

#[derive(Default)]
pub struct AzeroId {}

impl AzeroId {
    pub fn new() -> Self {
        AzeroId {}
    }
}

impl SignatureValidator for AzeroId {
    fn validate_signature(&self, account: String, nonce: String, signature: String) -> bool {
        let message = format!("<Bytes>{}</Bytes>", nonce);

        println!("account: {}", account);
        println!("nonce: {}", nonce);
        println!("signature: {}", signature);
        println!("message: {}", message);

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
        sp_core::sr25519::Pair::verify(&signature, message, &public_key)
    }
}

impl AzeroId {
    pub async fn is_nft_owner(
        &self,
        _contract: String,
        account: String,
        nft: Option<String>,
        chain: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let chain = Chain::from_str(&chain)?;
        match nft {
            Some(nft) => return Ok(Self::is_nft_owner_of(&chain, account, nft).await?),
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
        log::debug!(
            "is_nft_owner_of on chain {}: {} {}",
            chain.to_string(),
            account,
            domain
        );
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
    async fn test_wrong_tld() {
        let account: String = "5Esx8QLfERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY1fn".to_string();
        let domain: String = "chriamue.tzero".to_string();

        assert!(AzeroId::is_nft_owner_of(&Chain::Azero, account, domain)
            .await
            .is_err());
    }

    #[test]
    fn test_validate_signature() {
        let nonce = "random";
        let nonce = format!("<Bytes>{}</Bytes>", nonce);

        let signature = "0xead14bb8f93083c90d6a219b6a95a6f87e317fa0c680f7d30163935c229ceb5becf3610be148d9b0de2bfd9eb42c46bcfce78e1f24682cf0fc22a07cb7c55b8f";
        let account = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

        let account = account.to_string();
        let nonce = nonce.to_string();
        let signature = signature.to_string();

        let account_id = match AccountId32::from_str(&account) {
            Ok(acc) => acc,
            Err(_) => panic!("Invalid account"),
        };

        let public_key = Sr25519Public::from_raw(account_id.into());

        let signature_bytes = match Vec::from_hex(signature.trim_start_matches("0x")) {
            Ok(sig) => sig,
            Err(_) => panic!("Invalid signature"),
        };

        let signature = match Sr25519Signature::from_slice(&signature_bytes) {
            Some(sig) => sig,
            None => panic!("Invalid signature"),
        };
        let validated = sp_core::sr25519::Pair::verify(&signature, nonce, &public_key);
        assert_eq!(validated, true);
    }

    #[test]
    fn test_validate_signature2() {
        use subxt_signer::sr25519;

        let nonce = "random";
        let nonce = format!("<Bytes>{}</Bytes>", nonce);
        let signature = "0xead14bb8f93083c90d6a219b6a95a6f87e317fa0c680f7d30163935c229ceb5becf3610be148d9b0de2bfd9eb42c46bcfce78e1f24682cf0fc22a07cb7c55b8f";
        let account = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

        let account_id = match AccountId32::from_str(&account) {
            Ok(acc) => acc,
            Err(_) => panic!("Invalid account"),
        };

        let account = sr25519::PublicKey(account_id.into());
        let signature: [u8; 64] = hex::decode(signature.trim_start_matches("0x"))
            .unwrap()
            .try_into()
            .unwrap();
        let signature = sr25519::Signature(signature);

        assert!(sr25519::verify(&signature, nonce.as_bytes(), &account));
    }

    #[test]
    fn test_validate_signature3() {
        use sp_core::crypto::Ss58Codec;
        use subxt_signer::sr25519;

        let nonce = "random";
        let message = format!("<Bytes>{}</Bytes>", nonce);
        let alice = sr25519::dev::alice();

        let signature = alice.sign(message.as_bytes());

        assert!(sr25519::verify(
            &signature,
            message.as_bytes(),
            &alice.public_key()
        ));

        let account = AccountId32::from(alice.public_key().0);
        let account = account.to_ss58check();

        let signature = hex::encode(signature.0);

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
