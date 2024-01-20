use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use web3_login::authorize::AuthScope;
use web3_login::authorize::Authorize;
use web3_login::authorize::AuthorizeOutcome;
use web3_login::claims::ClaimsMutex;
use web3_login::config::Config;
use web3_login::jwk::JWKImpl;
use web3_login::prelude::*;
use web3_login::token::TokenImpl;
use web3_login::token::Tokens;
use web3_login::userinfo::UserInfoImpl;
use web3_login::well_known::WellKnownImpl;

use crate::authorize_impl::AuthorizeImpl;

pub fn create_server(config: Config) -> Server {
    let claims: ClaimsMutex = ClaimsMutex {
        standard_claims: Arc::new(Mutex::new(HashMap::new())),
        additional_claims: Arc::new(Mutex::new(HashMap::new())),
    };

    let tokens: Tokens = Tokens {
        muted: Arc::new(Mutex::new(HashMap::new())),
        bearer: Arc::new(Mutex::new(HashMap::new())),
    };

    let user_info: Arc<Box<dyn UserInfoTrait>> =
        Arc::new(Box::new(UserInfoImpl::new(claims.clone())));
    let jwk: Arc<Box<dyn JWKTrait>> = Arc::new(Box::new(JWKImpl::new(config.clone())));

    let well_known: Arc<Box<dyn WellKnownTrait>> =
        Arc::new(Box::new(WellKnownImpl::new(config.clone())));

    let token: Arc<Box<dyn TokenTrait>> = Arc::new(Box::new(TokenImpl::new(tokens.clone())));

    let authorize: Arc<Box<dyn AuthorizeTrait>> = Arc::new(Box::new(AuthorizeImpl::new(
        config.clone(),
        claims.clone(),
        tokens.clone(),
    )));
    Server {
        config,
        claims,
        tokens,
        user_info,
        jwk,
        well_known,
        token,
        authorize,
    }
}

#[cfg(test)]
mod tests {

    async fn test_authorize() {}
}
