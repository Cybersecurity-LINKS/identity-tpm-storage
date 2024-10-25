use crate::tpm_storage::TpmKeyId;

#[derive(Debug, thiserror::Error)]
pub enum TpmStorageError{
    #[error("Cannot access the TPM Device")]
    DeviceUnavailableError,
    #[error("Cannot create the key. Reason: {0}")]
    KeyGenerationError(String),
    #[error("Key storage error. Reason {0}")]
    KeyStorageError(String),
    #[error("Key not found")]
    KeyNotFound,
    #[error("InvalidAddress: {0}")]
    BadAddressError(String),
    #[error("Bad input value: {0}")]
    BadInput(String),
    #[error("Unexpected error: {0}")]
    UnexpectedBehaviour(String),
    #[error("Error encountered during sign operation: {0}")]
    SignatureError(String),
    #[error("Cannot delete the resource {handle}, reason: {reason}")]
    DeleteError{handle: TpmKeyId, reason: String},
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("The size {0} is not valid")]
    SizeError(usize)
}