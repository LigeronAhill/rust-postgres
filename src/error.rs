#[derive(Debug)]
pub enum AppError {
    SetGlobalDefaultError,
    FileError,
    ConfigError(String),
}

pub type AppResult<T> = std::result::Result<T, AppError>;

impl From<tracing::subscriber::SetGlobalDefaultError> for AppError {
    fn from(_err: tracing::subscriber::SetGlobalDefaultError) -> Self {
        AppError::SetGlobalDefaultError
    }
}
impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err.to_string())
    }
}
