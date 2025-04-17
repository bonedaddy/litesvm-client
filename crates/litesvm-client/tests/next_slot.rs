use {litesvm::LiteSVM, litesvm_client::SVMClient, solana_sdk::clock::Clock};

#[test]
fn test_next_slot() {
    let mut svm_client = SVMClient::new();

    let current_slot = AsRef::<LiteSVM>::as_ref(&svm_client)
        .get_sysvar::<Clock>()
        .slot;

    svm_client.next_slot(1);

    let new_slot = AsRef::<LiteSVM>::as_ref(&svm_client)
        .get_sysvar::<Clock>()
        .slot;

    assert!(new_slot > current_slot);
    assert_eq!(new_slot - 1, current_slot);
}
