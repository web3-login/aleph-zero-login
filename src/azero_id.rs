use crate::chain::Chain;
use crate::azero::get_owner as get_azero_owner;
use crate::tzero::get_owner as get_tzero_owner;

pub async fn is_nft_owner_of(
    chain: &Chain,
    account: String,
    domain: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    match chain {
        Chain::Azero => {
            match get_azero_owner(domain).await {
                Ok(owner) => Ok(owner == account),
                Err(e) => Err(e),
            }
        },
        Chain::AzeroTest => {
            match get_tzero_owner(domain).await {
                Ok(owner) => Ok(owner == account),
                Err(e) => Err(e),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_nft_owner() {
        let account: String = "5Esx8QLfERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY1fn".to_string();
        let domain: String = "chriamue.azero".to_string();

        let is_owner = is_nft_owner_of(&Chain::Azero, account, domain).await.unwrap();
        assert_eq!(is_owner, true);
    }

    #[tokio::test]
    async fn test_is_not_nft_owner() {
        let account: String = "5Esx8000ERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY000".to_string();
        let domain: String = "chriamue.azero".to_string();

        let is_owner = is_nft_owner_of(&Chain::Azero, account, domain).await.unwrap();
        assert_eq!(is_owner, false);
    }

    #[tokio::test]
    async fn test_wront_tld() {
        let account: String = "5Esx8QLfERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY1fn".to_string();
        let domain: String = "chriamue.tzero".to_string();
        assert!(is_nft_owner_of(&Chain::Azero, account, domain).await.is_err());
    }
}


