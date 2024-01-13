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
}
