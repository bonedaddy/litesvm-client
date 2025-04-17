# litesvm-client

`litesvm-client` provides a wrapper type around `LiteSVM` to help with usage of `LiteSVM`, in particular with loading of accoutns and programs.

Trait's are used to define how to load programs/accounts so that you may use multiple different sources.

For example you can load accounts from an RPC, and in-memory

## Usage

The following example can be used to load accounts from an RPC. For additional examples see `crates/litesvm-client/tests`

```rust
use {
    litesvm_client::{SVMClient, rpc_loader::RpcLoader, traits::{AccountLoader, AccountsLoader}},
    litesvm::LiteSVM,
};

let mut svm_client = SVMClient::new();

let rpc_loader = RpcLoader::new(
    "https://api.devnet.solana.com".to_string(),
    vec!["So11111111111111111111111111111111111111112"
        .parse()
        .unwrap()
    ],
);


svm_client.load_accounts(
    rpc_loader
    .load_accounts()
    .await
    .unwrap()
    .accounts()
).unwrap();

assert!(AsRef::<LiteSVM>::as_ref(&svm_client)
    .get_account(
        &"So11111111111111111111111111111111111111112"
            .parse()
            .unwrap()
    )
    .is_some());
```
