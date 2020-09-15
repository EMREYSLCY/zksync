pub use jsonrpc_core::types::response::Failure as RpcFailure;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network '{0}' is not supported")]
    NetworkNotSupported(String),
    #[error("Unable to decode server response")]
    MalformedResponse(String),
    #[error("RPC error: {0:?}")]
    RpcError(RpcFailure),
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Provided account credentials are incorrect")]
    IncorrectCredentials,
    #[error("Seed too short, must be at least 32 bytes long")]
    SeedTooShort,
    #[error("Token is not supported by zkSync")]
    UnknownToken,
    #[error("Incorrect address")]
    IncorrectAddress,

    #[error("Operation timeout")]
    OperationTimeout,
    #[error("Polling interval is too small")]
    PollingIntervalIsTooSmall,

    #[error("Signing error: {0}")]
    SigningError(SignerError),
    #[error("Missing required field for a transaction: {0}")]
    MissingRequiredField(String),

    #[error("Ethereum private key was not provided for this wallet")]
    NoEthereumPrivateKey,
}

#[derive(Debug, Error)]
pub enum SignerError {
    #[error("Ethereum private key required to perform an operation")]
    MissingEthPrivateKey,
    #[error("Signing failed: {0}")]
    SigningFailed(String),
    #[error("Signing key is not set in account")]
    NoSigningKey,
}
