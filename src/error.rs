///! Error and result module
use crate::{response::Response, signer::SignerError};
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// User request or Apple response JSON data was faulty.
    #[error("Error serializing to JSON: {0}")]
    SerializeError(#[from] serde_json::Error),

    /// A problem connecting to APNs servers.
    #[error("Error connecting to APNs: {0}")]
    ConnectionError(#[from] hyper::Error),

    /// Couldn't generate an APNs token with the given key.
    #[error("Error creating a signature: {0}")]
    SignerError(#[from] SignerError),

    /// APNs couldn't accept the notification. Contains
    /// [Response](response/struct.Response.html) with additional
    /// information.
    #[error(
        "Notification was not accepted by APNs (reason: {})",
        .0.error
            .as_ref()
            .map(|e| e.reason.to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    )]
    ResponseError(Response),

    /// Invalid option values given in
    /// [NotificationOptions](request/notification/struct.NotificationOptions.html)
    #[error("Invalid options for APNs payload: {0}")]
    InvalidOptions(String),

    /// Error reading the certificate or private key.
    #[error("Error in reading a certificate file: {0}")]
    ReadError(#[from] io::Error),

    #[error("Cert is missing a pub key")]
    CertMissingPubKey,

    #[error("Cert is missing a private key")]
    CertMissingPrivKey,

    /// Unexpected private key (only EC keys are supported).
    #[cfg(all(not(feature = "openssl"), feature = "ring"))]
    #[error("Unexpected private key: {0}")]
    UnexpectedKey(#[from] ring::error::KeyRejected),
}

#[cfg(feature = "openssl")]
impl From<openssl::error::ErrorStack> for Error {
    fn from(e: openssl::error::ErrorStack) -> Self {
        Self::SignerError(SignerError::OpenSSL(e))
    }
}
