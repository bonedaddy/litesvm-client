pub mod traits;
pub mod errors;

#[cfg(test)]
pub mod test;

use {
    errors::SVMClientError, litesvm::LiteSVM, traits::{AccountsLoader, ProgramsLoader}
};

pub struct SVMClient {
    svm: LiteSVM
}

impl SVMClient {
    /// Returns a new instance of [`SVMClient`] with a default [`LiteSVM`] runtime
    pub fn new() -> Self {
        Self {
            svm: LiteSVM::new()
        }
    }
    /// Returns a new instance of [`SVMClient`] with a pre-existing [`LiteSVM`] runtime
    /// 
    /// Useful if you want to provide a customized [`LiteSVM`] runtime
    pub fn with_litesvm(svm: LiteSVM) -> Self {
        Self {
            svm
        }
    }
    /// Sets one or more accounts in the [`LiteSVM`] runtime
    pub fn load_accounts(&mut self, accounts_loader: &mut dyn AccountsLoader) -> Result<(), SVMClientError> {
        for account in accounts_loader.accounts() {
            self.svm.set_account(
                account.pubkey(),
                account.account()
            )?;
        }
        Ok(())
    }
    /// Adds one or more programs in the [`LiteSVM`] runtime
    pub fn load_programs(&mut self, programs_loader: &mut dyn ProgramsLoader) {
        for program in programs_loader.programs() {
            self.svm.add_program(
                program.pubkey(),
                program.data()
            );
        }
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