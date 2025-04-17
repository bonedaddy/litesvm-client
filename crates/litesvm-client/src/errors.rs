use thiserror::Error;

#[derive(Debug, Error)]
pub enum SVMClientError {
    #[error("LiteSVM Error")]
    LiteSVM(#[from] litesvm::error::LiteSVMError),
}
