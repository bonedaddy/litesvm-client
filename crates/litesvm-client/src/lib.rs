//! `litesvm-client is a helper library for working with the litesvm crate.

pub mod errors;
pub mod traits;

#[cfg(feature = "rpc-loader")]
pub mod rpc_loader;

use {
    errors::SVMClientError,
    litesvm::LiteSVM,
    solana_sdk::sysvar::clock::Clock,
    traits::{AccountLoader, ProgramLoader},
};

/// SVMClient wraps [`LiteSVM`] and provides helper functions for common use cases
#[derive(Clone)]
pub struct SVMClient {
    svm: LiteSVM,
}

impl SVMClient {
    /// Returns a new instance of [`SVMClient`] with a default [`LiteSVM`] runtime
    pub fn new() -> Self {
        Self {
            svm: LiteSVM::new(),
        }
    }
    /// Returns a new instance of [`SVMClient`] with a pre-existing [`LiteSVM`] runtime
    ///
    /// Useful if you want to provide a customized [`LiteSVM`] runtime
    pub fn with_litesvm(svm: LiteSVM) -> Self {
        Self { svm }
    }
    /// Sets one or more accounts in the [`LiteSVM`] runtime
    pub fn load_accounts(
        &mut self,
        accounts_loader: Vec<Box<dyn AccountLoader>>,
    ) -> Result<(), SVMClientError> {
        for account in accounts_loader {
            self.svm.set_account(account.pubkey(), account.account())?;
        }
        Ok(())
    }
    /// Adds one or more programs in the [`LiteSVM`] runtime
    pub fn load_programs(
        &mut self,
        programs_loader: Vec<Box<dyn ProgramLoader>>,
    ) -> Result<(), SVMClientError> {
        for program in programs_loader {
            self.svm.add_program(program.pubkey(), &program.data())?;
        }
        Ok(())
    }
    /// Advances the timestamp reported by the [`Clock`] sysvar
    ///
    /// This does not update any other values in the [`Clock`] sysvar, and instead
    /// copies whatever the values currently reported are
    pub fn advance_time(&mut self, unix_timestamp: i64) {
        let clock: Clock = self.svm.get_sysvar();
        self.svm.set_sysvar(&Clock {
            unix_timestamp,
            slot: clock.slot,
            epoch_start_timestamp: clock.epoch_start_timestamp,
            epoch: clock.epoch,
            leader_schedule_epoch: clock.leader_schedule_epoch,
        });
    }
    /// Advances the slot by `slots_to_advance`
    ///
    /// # Arguments
    /// * `slots_to_advance` - number of slots to advance by
    pub fn next_slot(&mut self, slots_to_advance: u64) {
        self.svm
            .warp_to_slot(self.svm.get_sysvar::<Clock>().slot + slots_to_advance);
    }
}

impl AsMut<LiteSVM> for SVMClient {
    fn as_mut(&mut self) -> &mut LiteSVM {
        &mut self.svm
    }
}

impl AsRef<LiteSVM> for SVMClient {
    fn as_ref(&self) -> &LiteSVM {
        &self.svm
    }
}
