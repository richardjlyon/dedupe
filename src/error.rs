#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("network error")]
    NetworkError,

    #[error("serialisation error")]
    SerialisationError(#[from] serde_json::Error),

    #[error("file error")]
    FileError(#[from] std::io::Error),
}
