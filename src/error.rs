#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("network error")]
    NetworkError,

    #[error("exif decode error")]
    ExifError(#[from] exif::Error),

    #[error("missing datetime error")]
    DateTimeError,

    #[error("no dimension found")]
    DimensionError,
}
