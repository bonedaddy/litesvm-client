use solana_sdk::{account::Account, pubkey::Pubkey};

/// Trait used to load a single account
pub trait AccountLoader {
    /// Returns the public key of an account to load
    fn pubkey(&self) -> Pubkey;
    /// Returns the account to load
    fn account(&self) -> Account;
}

/// Trait used to load multiple accounts
pub trait AccountsLoader {
    /// Returns multiple [`AccountLoader`]
    fn accounts(&self) -> Vec<Box<dyn AccountLoader>>;
}

/// Trait used to load a single program
pub trait ProgramLoader {
    /// Returns the program address of a program to load
    fn pubkey(&self) -> Pubkey;
    /// Returns the program data
    fn data(&self) -> &[u8];
}

/// Trait used to load multiple programs
pub trait ProgramsLoader {
    /// Returns multiple [`ProgramLoader`]
    fn programs(&self) -> Vec<Box<dyn ProgramLoader>>;
}
