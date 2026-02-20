use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("upstream timeout")]
    UpstreamTimeout,
    #[error("upstream error: {0}")]
    UpstreamError(String),
    #[error("audit sink error: {0}")]
    AuditSinkError(String),
}
