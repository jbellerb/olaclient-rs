/// Result type returned from methods with olaclient `Errors`s.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur when talking to `olad`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// I/O errors, usually with the underlying connection.
    #[error("connection error: {0}")]
    Io(#[from] std::io::Error),
}
