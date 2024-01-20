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

impl AuthorizeTrait for AuthorizeImpl {
    fn authorize(
        &self,
        #[allow(unused_variables)] auth_scope: AuthScope,
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
        block_on(self.authorize_nft(
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
        ))
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
        if Url::parse(&redirect_uri).is_err() {
            return Ok(AuthorizeOutcome::Error(
                "wrong%20redirect%20uri".to_string(),
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

        let redirect_uri = Url::parse(&redirect_uri);

        if redirect_uri.is_err() {
            return Ok(AuthorizeOutcome::Error(
                "wrong%20redirect%20uri".to_string(),
            ));
        }

        let mut redirect_uri = redirect_uri.unwrap();

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
