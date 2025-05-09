use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Request timed out")]
    Timeout,

    #[error("Invalid URL")]
    InvalidURL,

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("HTTP request failed with status code: {0}")]
    Status(reqwest::StatusCode),

    #[error("Some other request error: {0}")]
    Other(reqwest::Error),
}

#[derive(Debug, Error)]
pub enum InstallationError {
    #[error("Failed to execute the installation {0}")]
    FailedToExecute(String),
    #[error("Minecraft version {0} was not found")]
    VersionNotFound(String),
    #[error("Installation {0} was not found")]
    InstallationNotFound(String),
}

#[derive(Debug, Error)]
pub enum JavaError {
    #[error("Invalid Java package type {0}")]
    InvalidPackageType(String),
    #[error("Java is Already installed")]
    AlreadyExists,
    #[error("Java version {0} was not found")]
    VersionNotFound(u16),
}

#[derive(Debug, Error)]
pub enum ConfigError {}

#[derive(Debug, Error)]
pub enum ZipExtractionError {
    #[error("Unsupported File Extension: {0}")]
    UnsupportedFileExt(String),
}

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Zip error: {0}")]
    ZipError(#[from] ZipError),

    #[error("Download error: {0}")]
    DownloadError(#[from] DownloadError),

    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Environmental variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Zip extraction error: {0}")]
    ZipExtractionError(#[from] ZipExtractionError),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("JSON serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Java error: {0}")]
    JavaError(#[from] JavaError),

    #[error("Installation error: {0}")]
    InstallationError(#[from] InstallationError),
}

impl From<reqwest::Error> for DownloadError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_builder() {
            DownloadError::InvalidURL
        } else if value.is_timeout() {
            DownloadError::Timeout
        } else if let Some(status) = value.status() {
            DownloadError::Status(status)
        } else {
            DownloadError::Other(value)
        }
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(value: reqwest::Error) -> Self {
        BackendError::DownloadError(DownloadError::from(value))
    }
}
