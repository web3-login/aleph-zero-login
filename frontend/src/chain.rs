use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Chain {
    Azero,
    AzeroTest,
}

impl Chain {
    pub fn get_url(&self) -> &str {
        match self {
            Chain::Azero => "wss://ws.azero.dev:443",
            Chain::AzeroTest => "wss://ws.test.azero.dev:443",
        }
    }

    pub fn get_contract(&self) -> &str {
        match self {
            Chain::Azero => "5FfRtDtpS3Vcr7BTChjPiQNrcAKu3VLv4E1NGF6ng6j3ZopJ",
            Chain::AzeroTest => "5HXjj3xhtRMqRYCRaXTDcVPz3Mez2XBruyujw6UEkvn8PCiA",
        }
    }

    pub fn get_tld(&self) -> &str {
        match self {
            Chain::Azero => "azero",
            Chain::AzeroTest => "tzero",
        }
    }
}

impl ToString for Chain {
    fn to_string(&self) -> String {
        match self {
            Chain::Azero => "Azero".to_string(),
            Chain::AzeroTest => "AzeroTest".to_string(),
        }
    }
}

impl FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Azero" => Ok(Chain::Azero),
            "AzeroTest" => Ok(Chain::AzeroTest),
            _ => Err("Unknown chain".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn main() {
        let chain = Chain::Azero;
        assert_eq!(chain.get_url(), "wss://ws.azero.dev:443");
        assert_eq!(
            chain.get_contract(),
            "5FfRtDtpS3Vcr7BTChjPiQNrcAKu3VLv4E1NGF6ng6j3ZopJ"
        );
        assert_eq!(chain.get_tld(), "azero");

        let chain = Chain::AzeroTest;
        assert_eq!(chain.get_url(), "wss://ws.test.azero.dev:443");
        assert_eq!(
            chain.get_contract(),
            "5HXjj3xhtRMqRYCRaXTDcVPz3Mez2XBruyujw6UEkvn8PCiA"
        );
        assert_eq!(chain.get_tld(), "tzero");
    }
}
