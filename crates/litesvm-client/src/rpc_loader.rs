use {
    crate::traits::{AccountLoader, AccountsLoader},
    anyhow::{anyhow, Context, Result},
    solana_account_decoder_client_types::UiAccountEncoding,
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcAccountInfoConfig},
    solana_sdk::{account::Account, pubkey::Pubkey},
};

/// RpcLoader allows loading accounts from an RPC
pub struct RpcLoader {
    pub rpc: RpcClient,
    pub accounts: Vec<Pubkey>,
}

/// RpcLoadedAccount denotes an account that has been retrieved from the rpc
#[derive(Clone)]
pub struct RpcLoadedAccount {
    pub key: Pubkey,
    pub account: Account,
}

impl RpcLoader {
    pub fn new(url: String, accounts: Vec<Pubkey>) -> Self {
        Self {
            rpc: RpcClient::new(url),
            accounts,
        }
    }
    pub fn with_client(rpc: RpcClient, accounts: Vec<Pubkey>) -> Self {
        Self { rpc, accounts }
    }
    /// Attempts to fetch all accounts from an RPC
    ///
    /// # Errors
    /// * If any of the accounts fail to load (ie do not exist)
    pub async fn load_accounts(&self) -> Result<Vec<RpcLoadedAccount>> {
        let mut loaded_accounts: Vec<RpcLoadedAccount> = Vec::with_capacity(self.accounts.len());

        for (idx, account) in self
            .rpc
            .get_multiple_accounts_with_config(
                &self.accounts,
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..Default::default()
                },
            )
            .await
            .with_context(|| "failed to load accounts")?
            .value
            .into_iter()
            .enumerate()
        {
            let account =
                account.with_context(|| anyhow!("failed to load account at idx {idx}"))?;
            let key = self
                .accounts
                .get(idx)
                .with_context(|| anyhow!("failed to find account at idx {idx}"))?;
            loaded_accounts.push(RpcLoadedAccount { key: *key, account })
        }

        Ok(loaded_accounts)
    }
}

impl AccountLoader for RpcLoadedAccount {
    fn account(&self) -> solana_sdk::account::Account {
        self.account.clone()
    }
    fn pubkey(&self) -> Pubkey {
        self.key
    }
}

impl AccountsLoader for Vec<RpcLoadedAccount> {
    fn accounts(&self) -> Vec<Box<dyn AccountLoader>> {
        self.iter()
            .map(|acc| Box::new(acc.clone()) as Box<dyn AccountLoader>)
            .collect()
    }
}
