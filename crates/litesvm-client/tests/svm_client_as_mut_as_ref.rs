use litesvm::LiteSVM;
use solana_sdk::{account::Account, pubkey::Pubkey};

use litesvm_client::SVMClient;

#[test]
fn test_svm_client_as_mut_as_ref() {
    let mut svm_client = SVMClient::new();


    let account_key = Pubkey::new_unique();
    let owner_key = Pubkey::new_unique();

    {
        let svm: &mut LiteSVM = svm_client.as_mut();


        svm.set_account(
            account_key,
            Account {
                lamports: 1,
                data: b"foobar".to_vec(),
                owner: owner_key,
                executable: false,
                rent_epoch: 1337
            }
        ).unwrap();
    }

    {
        let svm: &LiteSVM = svm_client.as_ref();

        let account = svm.get_account(&account_key).unwrap();

        assert_eq!(account.data, b"foobar".to_vec());
        assert_eq!(account.owner, owner_key);
        assert!(!account.executable);
        assert_eq!(account.rent_epoch, 1337);
    }
}