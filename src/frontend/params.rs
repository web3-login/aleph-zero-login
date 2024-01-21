use super::signature::Signature;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Params {
    pub client_id: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub response_type: Option<String>,
    pub response_mode: Option<String>,
    pub redirect_uri: Option<String>,
    pub realm: Option<String>,
    pub signature: Option<String>,
    pub account: Option<String>,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            redirect_uri: Some("https%3A%2F%2Foidcdebugger.com%2Fdebug".to_string()),
            state: None,
            nonce: Some("random".to_string()),
            response_type: Some("code+id_token".to_string()),
            response_mode: Some("code+id_token".to_string()),
            realm: Some("AzeroTest".to_string()),
            client_id: Some("none".to_string()),
            signature: None,
            account: None,
        }
    }
}

impl Params {
    pub fn merge_signature(&mut self, signature: &Signature) {
        self.account = Some(signature.account.clone());
        self.signature = Some(signature.signature.clone());
    }

    pub fn merge_realm(&mut self, realm: &str) {
        self.realm = Some(realm.to_string());
    }

    pub fn merge_default(&mut self) {
        let default = Params::default();
        if self.redirect_uri.is_none() {
            self.redirect_uri = default.redirect_uri;
        }
        if self.state.is_none() {
            self.state = default.state;
        }
        if self.nonce.is_none() {
            self.nonce = default.nonce;
        }
        if self.response_type.is_none() {
            self.response_type = default.response_type;
        }
        if self.response_mode.is_none() {
            self.response_mode = default.response_mode;
        }
        if self.realm.is_none() {
            self.realm = default.realm;
        }
        if self.client_id.is_none() {
            self.client_id = default.client_id;
        }
        if self.signature.is_none() {
            self.signature = default.signature;
        }
        if self.account.is_none() {
            self.account = default.account;
        }
    }
}
