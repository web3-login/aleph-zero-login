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
}

impl Default for Params {
    fn default() -> Self {
        Params {
            redirect_uri: Some("https%3A%2F%2Foidcdebugger.com%2Fdebug".to_string()),
            state: None,
            nonce: None,
            response_type: Some("code+id_token".to_string()),
            response_mode: Some("code+id_token".to_string()),
            realm: Some("AzeroTest".to_string()),
            client_id: None,
        }
    }
}
