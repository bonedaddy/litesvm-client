use litesvm::LiteSVM;
use solana_sdk::{account::Account, pubkey::Pubkey};

use litesvm_client::{traits::{AccountLoader, AccountsLoader}, SVMClient};

struct ExampleAccountsLoader {
    accounts: Vec<ExampleAccount>
}

struct ExampleAccount {
    key: Pubkey,
    account: Account,
}


impl AccountsLoader for ExampleAccountsLoader {
    fn accounts(&self) -> Vec<&dyn AccountLoader> {
        self.accounts.iter().map(|acct| acct as &dyn AccountLoader).collect()
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

#[test]
fn test_svm_client_accounts_loader() {
    let mut svm_client = SVMClient::new();

    let account_one_key = Pubkey::new_unique();
    let account_one_data = b"foo".to_vec();

    let account_two_key = Pubkey::new_unique();
    let account_two_data = b"foobar".to_vec();


    let accounts_loader = ExampleAccountsLoader {
        accounts: vec![
            ExampleAccount {
                key: account_one_key,
                account: Account {
                    data: account_one_data,
                    ..Default::default()
                }
            },
            ExampleAccount {
                key: account_two_key,
                account: Account {
                    data: account_two_data,
                    ..Default::default()
                }
            }
        ],
    };


    svm_client.load_accounts(accounts_loader).unwrap();

    let account_one = AsRef::<LiteSVM>::as_ref(&svm_client).get_account(&account_one_key).unwrap();
    let account_two = AsRef::<LiteSVM>::as_ref(&svm_client).get_account(&account_two_key).unwrap();

    assert_eq!(account_one.data, b"foo".to_vec());
    assert_eq!(account_two.data, b"foobar".to_vec());
}