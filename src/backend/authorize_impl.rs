use super::nft_authorize::NFTAuthorize;
use futures::executor::block_on;
use openidconnect::TokenResponse;
use openidconnect::{AccessToken, AuthorizationCode};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use url::Url;
use uuid::Uuid;
use web3_login::authorize::AuthScope;
use web3_login::authorize::Authorize;
use web3_login::authorize::AuthorizeError;
use web3_login::authorize::AuthorizeOutcome;
use web3_login::claims::ClaimsMutex;
use web3_login::config::Config;
use web3_login::jwk::JWKImpl;
use web3_login::prelude::*;
use web3_login::token::TokenImpl;
use web3_login::token::Tokens;
use web3_login::userinfo::UserInfoImpl;
use web3_login::well_known::WellKnownImpl;
use web3_login::{
    claims::{additional_claims, standard_claims},
    config::{get_chain_id, get_node},
    token::token,
    traits::AuthorizeTrait,
};

pub struct AuthorizeImpl {
    pub config: Config,
    pub claims: ClaimsMutex,
    pub tokens: Tokens,
}

impl AuthorizeImpl {
    pub fn new(config: Config, claims: ClaimsMutex, tokens: Tokens) -> Self {
        Self {
            config,
            claims,
            tokens,
        }
    }
}

#[async_trait::async_trait]
impl AuthorizeTrait for AuthorizeImpl {
    async fn authorize(
        &self,
        auth_scope: AuthScope,
        realm: Option<String>,
        client_id: String,
        redirect_uri: String,
        state: Option<String>,
        response_type: Option<String>,
        response_mode: Option<String>,
        nonce: Option<String>,
        account: Option<String>,
        signature: Option<String>,
        chain_id: Option<String>,
        contract: Option<String>,
    ) -> Result<AuthorizeOutcome, Box<dyn std::error::Error>> {
        self.authorize_nft(
            realm.unwrap_or_else(|| "default".into()),
            client_id,
            redirect_uri,
            state,
            response_type,
            response_mode,
            nonce,
            account,
            signature,
            chain_id,
            contract,
        )
        .await
    }
}

impl AuthorizeImpl {
    async fn authorize_nft(
        &self,
        realm: String,
        client_id: String,
        redirect_uri: String,
        state: Option<String>,
        response_type: Option<String>,
        response_mode: Option<String>,
        nonce: Option<String>,
        account: Option<String>,
        signature: Option<String>,
        chain_id: Option<String>,
        contract: Option<String>,
    ) -> Result<AuthorizeOutcome, Box<dyn std::error::Error>> {
        let redirect_uri: String = urlencoding::decode(&redirect_uri)?.into();
        log::debug!("redirect_uri: {}", redirect_uri);
        log::debug!("realm: {}", realm);
        log::debug!("client_id: {}", client_id);
        log::debug!("state: {:?}", state);
        log::debug!("response_type: {:?}", response_type);
        log::debug!("signature: {:?}", signature);
        log::debug!("nonce: {:?}", nonce);
        log::debug!("account: {:?}", account);

        if Url::parse(&redirect_uri).is_err() {
            return Ok(AuthorizeOutcome::Error(
                format!("wrong%20redirect%20uri {}", redirect_uri).to_string(),
            ));
        }

        if account.is_none() {
            let mut url = Url::parse(&format!("{}/", self.config.frontend_host)).unwrap();
            url.query_pairs_mut()
                .clear()
                .append_pair("client_id", &client_id)
                .append_pair("state", &state.unwrap_or_default())
                .append_pair("nonce", &nonce.unwrap_or_default())
                .append_pair("response_type", &response_type.unwrap_or_default())
                .append_pair("response_mode", &response_mode.unwrap_or_default())
                .append_pair("redirect_uri", &redirect_uri)
                .append_pair("realm", &realm.clone())
                .append_pair(
                    "chain_id",
                    &chain_id.clone().unwrap_or_else(|| realm.clone()),
                )
                .append_pair("contract", &contract.unwrap_or_else(|| client_id.clone()));
            return Ok(AuthorizeOutcome::RedirectNeeded(url.to_string()));
        };

        let contract = contract.unwrap_or_else(|| client_id.clone());

        let realm_or_chain_id = match realm.as_str() {
            "default" => chain_id.clone().unwrap_or_else(|| "default".into()),
            _ => realm.clone(),
        };

        let node_provider = get_node(&self.config, &realm_or_chain_id);

        let authorize = NFTAuthorize {
            account: account.clone(),
            nonce: nonce.clone(),
            signature: signature.clone(),
            node: node_provider.clone(),
            realm: realm_or_chain_id.clone(),
            contract: contract.clone(),
            nft: state.clone(),
        };

        match authorize.authorize().await {
            Ok(_) => (),
            Err(err) => match err {
                AuthorizeError::AccountError => {
                    return Ok(AuthorizeOutcome::Error("account%20missing".to_string()))
                }
                AuthorizeError::NonceError => {
                    return Ok(AuthorizeOutcome::Error("nonce%20missing".to_string()))
                }
                AuthorizeError::SignatureError => {
                    return Ok(AuthorizeOutcome::Error("signature%20missing".to_string()))
                }
                AuthorizeError::NFTError => {
                    return Ok(AuthorizeOutcome::Denied("access%20denied".to_string()))
                }
            },
        };

        let parsed_redirect_uri = Url::parse(&redirect_uri);

        if parsed_redirect_uri.is_err() {
            return Ok(AuthorizeOutcome::Error(
                format!("wrong%20redirect%20uri {}", redirect_uri).to_string(),
            ));
        }

        let mut redirect_uri = parsed_redirect_uri.unwrap();

        let access_token = AccessToken::new(Uuid::new_v4().to_string());
        let code = AuthorizationCode::new(Uuid::new_v4().to_string());
        let chain_id = get_chain_id(&self.config, &realm_or_chain_id);

        let standard_claims = standard_claims(&account.clone().unwrap());

        let node_provider_url = Url::parse(&node_provider).unwrap();
        let node_provider_host = node_provider_url.host().unwrap().to_string();

        let additional_claims = additional_claims(
            &account.unwrap(),
            &nonce.clone().unwrap(),
            &signature.unwrap(),
            &chain_id,
            &node_provider_host,
            &contract,
        );

        self.claims
            .standard_claims
            .try_lock()
            .unwrap()
            .insert(access_token.secret().clone(), standard_claims.clone());

        self.claims
            .additional_claims
            .try_lock()
            .unwrap()
            .insert(access_token.secret().clone(), additional_claims.clone());

        let token = token(
            &self.config,
            client_id,
            nonce,
            standard_claims,
            additional_claims,
            access_token.clone(),
            code.clone(),
        )
        .await;

        let id_token = token.id_token().unwrap().to_string();

        self.tokens
            .bearer
            .try_lock()
            .unwrap()
            .insert(code.secret().clone(), access_token.secret().clone());
        self.tokens
            .muted
            .try_lock()
            .unwrap()
            .insert(access_token.secret().clone(), token);

        if let Some(response_type) = response_type {
            if response_type.contains("code") {
                redirect_uri
                    .query_pairs_mut()
                    .append_pair("code", code.secret());
            }
            if response_type.contains("id_token") || response_type.contains("token") {
                redirect_uri
                    .query_pairs_mut()
                    .append_pair("id_token", &id_token);
            }
        } else {
            redirect_uri
                .query_pairs_mut()
                .append_pair("code", code.secret());
        };

        if let Some(state) = state {
            redirect_uri.query_pairs_mut().append_pair("state", &state);
        };

        Ok(AuthorizeOutcome::RedirectNeeded(redirect_uri.to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_authorize() {
        let nonce = "random";
        let signature = "0x264fb4760958f40fc4886152f6a8e49615c7750341ce050849f7592e800f925aca43cc161c7b94250c5ecc89b8ab528132ceae69697ea85dd9822a81b58f568f";
        let account = "5Esx8QLfERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY1fn";
        let state = "chriamue.tzero";

        let mut config = Config::default();
        config.eddsa_pem = Some("-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIGWObwgsl5OQvHbjsTxMxuhnLaAXysh/+2AKHYXXVfoK\n-----END PRIVATE KEY-----".into());

        let claims: ClaimsMutex = ClaimsMutex {
            standard_claims: Arc::new(Mutex::new(HashMap::new())),
            additional_claims: Arc::new(Mutex::new(HashMap::new())),
        };

        let tokens: Tokens = Tokens {
            muted: Arc::new(Mutex::new(HashMap::new())),
            bearer: Arc::new(Mutex::new(HashMap::new())),
        };

        let authorize = AuthorizeImpl::new(config, claims, tokens);

        let outcome = authorize
            .authorize(
                AuthScope::Account,
                Some("AzeroTest".into()),
                "client_id".into(),
                "http://localhost:3000".into(),
                Some(state.into()),
                Some("code".into()),
                Some("query".into()),
                Some(nonce.into()),
                Some(account.into()),
                Some(signature.into()),
                Some("chain_id".into()),
                Some("contract".into()),
            )
            .await;

        assert_eq!(outcome.is_ok(), true);

        let outcome = outcome.unwrap();

        match outcome {
            AuthorizeOutcome::RedirectNeeded(url) => {
                assert_eq!(url.contains("http://localhost:3000"), true);
                assert_eq!(url.contains("state=chriamue.tzero"), true);
                assert!(url.contains("code="));
                assert_eq!(url.contains("nonce="), false);
                assert_eq!(url.contains("response_type="), false);
                assert_eq!(url.contains("response_mode="), false);
                assert_eq!(url.contains("redirect_uri="), false);
                assert_eq!(url.contains("realm="), false);
                assert_eq!(url.contains("chain_id="), false);
                assert_eq!(url.contains("contract="), false);
            }
            AuthorizeOutcome::Denied(message) => panic!("should not denied: {}", message),
            AuthorizeOutcome::Error(err) => panic!("should not error: {}", err),
            AuthorizeOutcome::Success(_) => panic!("should not success"),
        }
    }
}
