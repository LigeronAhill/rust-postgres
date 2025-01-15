#[derive(Debug)]
pub enum AppError {
    SetGlobalDefaultError,
    FileError,
}

pub type AppResult<T> = std::result::Result<T, AppError>;

impl From<tracing::subscriber::SetGlobalDefaultError> for AppError {
    fn from(_err: tracing::subscriber::SetGlobalDefaultError) -> Self {
        AppError::SetGlobalDefaultError
    }
}
