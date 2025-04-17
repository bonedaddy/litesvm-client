use {
    litesvm::LiteSVM,
    litesvm_client::{
        rpc_loader::RpcLoader,
        traits::{AccountLoader, AccountsLoader},
        SVMClient,
    },
    solana_sdk::{account::Account, pubkey::Pubkey},
};

struct ExampleAccountsLoader {
    accounts: Vec<ExampleAccount>,
}

#[derive(Clone)]
struct ExampleAccount {
    key: Pubkey,
    account: Account,
}

impl AccountsLoader for ExampleAccountsLoader {
    fn accounts(&self) -> Vec<Box<dyn AccountLoader>> {
        self.accounts
            .iter()
            .map(|acct| Box::new(acct.clone()) as Box<dyn AccountLoader>)
            .collect()
    }
}

impl AccountLoader for ExampleAccount {
    fn account(&self) -> Account {
        self.account.clone()
    }
    fn pubkey(&self) -> Pubkey {
        self.key
    }
}

#[tokio::test]
async fn test_svm_client_accounts_loader() {
    // tests loading accounts from rpc, and in-memory

    let mut svm_client = SVMClient::new();

    let rpc_loader = RpcLoader::new(
        "https://api.devnet.solana.com".to_string(),
        vec!["So11111111111111111111111111111111111111112"
            .parse()
            .unwrap()],
    );

    let mut loaded_accounts = rpc_loader.load_accounts().await.unwrap().accounts();

    let account_one_key = Pubkey::new_unique();
    let account_one_data = b"foo".to_vec();

    let account_two_key = Pubkey::new_unique();
    let account_two_data = b"foobar".to_vec();

    loaded_accounts.append(
        &mut ExampleAccountsLoader {
            accounts: vec![
                ExampleAccount {
                    key: account_one_key,
                    account: Account {
                        data: account_one_data,
                        ..Default::default()
                    },
                },
                ExampleAccount {
                    key: account_two_key,
                    account: Account {
                        data: account_two_data,
                        ..Default::default()
                    },
                },
            ],
        }
        .accounts(),
    );

    svm_client.load_accounts(loaded_accounts).unwrap();

    assert!(AsRef::<LiteSVM>::as_ref(&svm_client)
        .get_account(
            &"So11111111111111111111111111111111111111112"
                .parse()
                .unwrap()
        )
        .is_some());

    let account = AsRef::<LiteSVM>::as_ref(&svm_client)
        .get_account(&account_one_key)
        .unwrap();
    assert_eq!(account.data, b"foo".to_vec());

    let account = AsRef::<LiteSVM>::as_ref(&svm_client)
        .get_account(&account_two_key)
        .unwrap();
    assert_eq!(account.data, b"foobar".to_vec());
}
