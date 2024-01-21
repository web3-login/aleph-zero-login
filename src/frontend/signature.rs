use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Signature {
    pub account: String,
    pub signature: String,
}
