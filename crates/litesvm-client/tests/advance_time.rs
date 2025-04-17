use {litesvm::LiteSVM, litesvm_client::SVMClient, solana_sdk::clock::Clock};

#[test]
fn test_advance_time() {
    let mut svm_client = SVMClient::new();

    let current_time: i64 = AsRef::<LiteSVM>::as_ref(&svm_client)
        .get_sysvar::<Clock>()
        .unix_timestamp;

    svm_client.advance_time(current_time + 1);

    let new_time: i64 = AsRef::<LiteSVM>::as_ref(&svm_client)
        .get_sysvar::<Clock>()
        .unix_timestamp;

    assert!(new_time > current_time);
    assert_eq!(new_time, current_time + 1);
}
