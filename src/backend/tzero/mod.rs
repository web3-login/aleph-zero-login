#![cfg(not(target_arch = "wasm32"))]

use crate::chain::Chain;
use aleph_client::Connection;
use ink_wrapper_types::util::ToAccountId as _;
use sp_core::crypto::AccountId32;
use sp_core::crypto::Ss58Codec;
use std::str::FromStr;

mod router_contract;
pub use router_contract::Instance;

pub async fn get_owner(domain: String) -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new(Chain::AzeroTest.get_url()).await;

    let contract: ink_primitives::AccountId =
        AccountId32::from_str(Chain::AzeroTest.get_contract())
            .unwrap()
            .to_account_id();

    let contract = Instance::from(contract);
    let owner_result = contract.get_address(&conn, domain).await?;

    let owner = match owner_result {
        Ok(Ok(owner_account)) => owner_account,
        Ok(Err(contract_error)) => {
            return Err(format!("Contract method error: {:?}", contract_error).into());
        }
        Err(lang_error) => {
            return Err(format!("Ink language error: {:?}", lang_error).into());
        }
    };

    let owner_slice: &[u8] = owner.as_ref();
    if owner_slice.len() == 32 {
        let mut owner_bytes = [0u8; 32];
        owner_bytes.copy_from_slice(owner_slice);

        let account_id: AccountId32 = AccountId32::from(owner_bytes);

        let owner_ss58 = account_id.to_ss58check();

        Ok(owner_ss58)
    } else {
        Err("Invalid AccountId length".into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let account: &str = "5Esx8QLfERemJmBmhZ9aJDgBmw69vLaE6rN5FNx3VPZDY1fn";
        let domain: String = "chriamue.tzero".to_string();

        let address = get_owner(domain).await?;

        assert_eq!(address, account);

        Ok(())
    }
}
