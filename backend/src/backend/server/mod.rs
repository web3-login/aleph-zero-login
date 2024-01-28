#![cfg(not(target_arch = "wasm32"))]

use crate::backend::authorize_impl::AuthorizeImpl;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use web3_login::claims::ClaimsMutex;
use web3_login::config::Config;
use web3_login::jwk::JWKImpl;
use web3_login::prelude::*;
use web3_login::server::routes::get_frontend;
use web3_login::server::routes::get_providers;
use web3_login::server::routes::get_realms;
use web3_login::token::TokenImpl;
use web3_login::token::Tokens;
use web3_login::userinfo::UserInfoImpl;
use web3_login::well_known::WellKnownImpl;

pub fn router(app: Server) -> Result<Router, Box<dyn Error>> {
    let cors = CorsLayer::new().allow_origin(Any);
    let router = Router::new()
        .route("/frontend", get(get_frontend))
        .route("/providers", get(get_providers))
        .route("/realms", get(get_realms))
        .nest_service("/example", ServeFile::new("dist/index.html"))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .nest_service("/index.css", ServeDir::new("static/index.css"))
        .nest_service("/index.js", ServeDir::new("static/index.js"))
        .nest_service("/400.html", ServeFile::new("static/400.html"))
        .nest_service("/401.html", ServeFile::new("static/401.html"));
    //.route("/index.html", get(get_index));

    let router = router
        .nest("/", oidc_routes())
        .nest("/:realm/", oidc_routes())
        .nest("/account", oidc_routes())
        .nest("/account/:realm/", oidc_routes());

    let router = router.layer(cors);

    Ok(router.with_state(app))
}

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
