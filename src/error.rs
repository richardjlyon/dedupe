#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("network error")]
    NetworkError,

    #[error("exif decode error")]
    ExifError(#[from] exif::Error),

    #[error("no datetime found")]
    DateTimeError,
}
