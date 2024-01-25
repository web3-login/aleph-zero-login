use scale::Encode as _;

// This file was auto-generated with ink-wrapper (https://crates.io/crates/ink-wrapper).
#[allow(dead_code)]
pub const CODE_HASH: [u8; 32] = [
    99, 48, 140, 19, 17, 175, 68, 187, 71, 8, 35, 39, 195, 1, 252, 229, 89, 202, 40, 246, 187, 1,
    10, 97, 185, 136, 77, 146, 70, 116, 106, 191,
];
#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
pub enum Error {
    NotAdmin(),
    InvalidRegistryAddress(),
    TldAlreadyInUse(String),
    TldNotFound(String),
    CouldNotResolveDomain(),
    InvalidDomainName(),
    EmptyList(),
}

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
pub enum NoChainExtension {}

pub mod event {
    #[allow(dead_code, clippy::large_enum_variant)]
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    pub enum Event {}
}
#[derive(Debug, Clone, Copy)]
pub struct Instance {
    account_id: ink_primitives::AccountId,
}
impl From<ink_primitives::AccountId> for Instance {
    fn from(account_id: ink_primitives::AccountId) -> Self {
        Self { account_id }
    }
}
impl From<Instance> for ink_primitives::AccountId {
    fn from(instance: Instance) -> Self {
        instance.account_id
    }
}
impl ink_wrapper_types::EventSource for Instance {
    type Event = event::Event;
}
impl Instance {
    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn new<TxInfo, E, C: ink_wrapper_types::SignedConnection<TxInfo, E>>(
        conn: &C,
        salt: Vec<u8>,
        admin: ink_primitives::AccountId,
    ) -> Result<Self, E> {
        let data = {
            let mut data = vec![155, 174, 157, 94];
            admin.encode_to(&mut data);
            data
        };
        let account_id = conn.instantiate(CODE_HASH, salt, data).await?;
        Ok(Self {
            account_id: account_id,
        })
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn add_registry<TxInfo, E, C: ink_wrapper_types::SignedConnection<TxInfo, E>>(
        &self,
        conn: &C,
        tld: Vec<String>,
        registry_addr: ink_primitives::AccountId,
    ) -> Result<TxInfo, E> {
        let data = {
            let mut data = vec![201, 151, 239, 227];
            tld.encode_to(&mut data);
            registry_addr.encode_to(&mut data);
            data
        };
        conn.exec(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn update_registry<TxInfo, E, C: ink_wrapper_types::SignedConnection<TxInfo, E>>(
        &self,
        conn: &C,
        tld: Vec<String>,
        registry_addr: ink_primitives::AccountId,
    ) -> Result<TxInfo, E> {
        let data = {
            let mut data = vec![86, 45, 232, 107];
            tld.encode_to(&mut data);
            registry_addr.encode_to(&mut data);
            data
        };
        conn.exec(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn remove_registry_address<
        TxInfo,
        E,
        C: ink_wrapper_types::SignedConnection<TxInfo, E>,
    >(
        &self,
        conn: &C,
        registry_addr: ink_primitives::AccountId,
    ) -> Result<TxInfo, E> {
        let data = {
            let mut data = vec![204, 120, 174, 173];
            registry_addr.encode_to(&mut data);
            data
        };
        conn.exec(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn get_all_registries<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>(
        &self,
        conn: &C,
    ) -> Result<
        Result<Vec<(ink_primitives::AccountId, Vec<String>)>, ink_wrapper_types::InkLangError>,
        E,
    > {
        let data = vec![230, 218, 123, 240];
        conn.read(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn get_all_tlds<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>(
        &self,
        conn: &C,
    ) -> Result<Result<Vec<String>, ink_wrapper_types::InkLangError>, E> {
        let data = vec![241, 167, 175, 65];
        conn.read(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn get_associated_tlds<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>(
        &self,
        conn: &C,
        registry_addr: ink_primitives::AccountId,
    ) -> Result<Result<Vec<String>, ink_wrapper_types::InkLangError>, E> {
        let data = {
            let mut data = vec![203, 24, 248, 33];
            registry_addr.encode_to(&mut data);
            data
        };
        conn.read(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn get_registry<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>(
        &self,
        conn: &C,
        tld: String,
    ) -> Result<Result<Option<ink_primitives::AccountId>, ink_wrapper_types::InkLangError>, E> {
        let data = {
            let mut data = vec![21, 165, 210, 10];
            tld.encode_to(&mut data);
            data
        };
        conn.read(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn get_address<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>(
        &self,
        conn: &C,
        domain: String,
    ) -> Result<Result<Result<ink_primitives::AccountId, Error>, ink_wrapper_types::InkLangError>, E>
    {
        let data = {
            let mut data = vec![210, 89, 247, 186];
            domain.encode_to(&mut data);
            data
        };
        conn.read(self.account_id, data).await
    }

    ///  @returns list of (registry-address, primary-domain) for given account #[allow(dead_code, clippy::too_many_arguments)] pub async fn get_primary_domains<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>( &self, conn: &C, account: ink_primitives::AccountId<>,tld: Option<String>, ) -> Result<Result<Vec<(ink_primitives::AccountId<>, String)>, ink_wrapper_types::InkLangError<>>, E> { let data = { let mut data = vec![223, 58, 53, 142]; account.encode_to(&mut data);tld.encode_to(&mut data); data }; conn.read(self.account_id, data).await }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn get_admin<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>(
        &self,
        conn: &C,
    ) -> Result<Result<ink_primitives::AccountId, ink_wrapper_types::InkLangError>, E> {
        let data = vec![87, 184, 168, 167];
        conn.read(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn get_pending_admin<TxInfo, E, C: ink_wrapper_types::Connection<TxInfo, E>>(
        &self,
        conn: &C,
    ) -> Result<Result<Option<ink_primitives::AccountId>, ink_wrapper_types::InkLangError>, E> {
        let data = vec![188, 211, 29, 118];
        conn.read(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn transfer_ownership<
        TxInfo,
        E,
        C: ink_wrapper_types::SignedConnection<TxInfo, E>,
    >(
        &self,
        conn: &C,
        account: Option<ink_primitives::AccountId>,
    ) -> Result<TxInfo, E> {
        let data = {
            let mut data = vec![16, 126, 51, 234];
            account.encode_to(&mut data);
            data
        };
        conn.exec(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn accept_ownership<TxInfo, E, C: ink_wrapper_types::SignedConnection<TxInfo, E>>(
        &self,
        conn: &C,
    ) -> Result<TxInfo, E> {
        let data = vec![181, 91, 233, 240];
        conn.exec(self.account_id, data).await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn upgrade_contract<TxInfo, E, C: ink_wrapper_types::SignedConnection<TxInfo, E>>(
        &self,
        conn: &C,
        code_hash: [u8; 32],
    ) -> Result<TxInfo, E> {
        let data = {
            let mut data = vec![19, 69, 84, 61];
            code_hash.encode_to(&mut data);
            data
        };
        conn.exec(self.account_id, data).await
    }
}
