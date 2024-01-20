use super::azero;
use super::azero_id::AzeroId;
use crate::chain;
use async_trait::async_trait;
use web3_login::authorize::Authorize;
use web3_login::authorize::AuthorizeError;
use web3_login::nft_owner::NFTOwner;
use web3_login::signature_validator::SignatureValidator;
use web3_login::web3::is_nft_owner_of;
use web3_login::web3::validate_signature;

use super::azero_id;

pub struct NFTAuthorize {
    pub account: Option<String>,
    pub nonce: Option<String>,
    pub signature: Option<String>,
    pub node: String,
    pub realm: String,
    pub contract: String,
    pub nft: Option<String>,
}

impl NFTAuthorize {
    async fn check_nft(&self) -> Result<(), AuthorizeError> {
        let account = self.get_account().as_ref().unwrap().to_string();
        let nonce = self.get_nonce().as_ref().unwrap().to_string();
        let signature = self.get_signature().as_ref().unwrap().to_string();
        let contract = self.contract.clone();

        let chain = chain::Chain::Azero.to_string();
        let nft = self.nft.clone();

        let azero_id = AzeroId::new();
        match azero_id.is_nft_owner(contract, account, nft, chain) {
            Ok(true) => Ok(()),
            Ok(false) => Err(AuthorizeError::NFTError),
            Err(e) => Err(AuthorizeError::NFTError),
        }
    }
}

#[async_trait]
impl Authorize for NFTAuthorize {
    fn get_account(&self) -> &Option<String> {
        &self.account
    }
    fn get_nonce(&self) -> &Option<String> {
        &self.nonce
    }
    fn get_signature(&self) -> &Option<String> {
        &self.signature
    }

    fn check_signature(&self) -> Result<(), AuthorizeError> {
        match self.get_signature() {
            Some(_) => (),
            None => return Err(AuthorizeError::SignatureError),
        };
        let account = self.get_account().as_ref().unwrap().to_string();
        let nonce = self.get_nonce().as_ref().unwrap().to_string();
        let signature = self.get_signature().as_ref().unwrap().to_string();

        let azero_id = AzeroId::new();

        match azero_id.validate_signature(account, nonce, signature) {
            true => Ok(()),
            false => Err(AuthorizeError::SignatureError),
        }
    }

    async fn authorize(&self) -> Result<(), AuthorizeError> {
        self.check_account()?;
        self.check_nonce()?;
        self.check_signature()?;
        self.check_nft().await?;
        Ok(())
    }
}
