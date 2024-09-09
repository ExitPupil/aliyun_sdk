use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http client error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("internal error occurred: {0}")]
    Internal(String),

    #[error("reach quato limit: {0}")]
    QuotaLimit(String),

    #[error("signature issue: {0}")]
    SignatureIssue(String),

    #[error("insufficient balance: {0}")]
    OutOfService(String),

    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
}
