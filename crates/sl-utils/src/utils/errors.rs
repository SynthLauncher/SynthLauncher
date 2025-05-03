use zip::result::ZipError;

#[derive(Debug)]
pub enum DownloadError {
    Timeout,
    InvalidURL,
    IO(std::io::Error),
    Status(reqwest::StatusCode),
    Other(reqwest::Error),
}

#[derive(Debug)]
pub enum BackendError {
    ZipError(ZipError),
    DownloadError(DownloadError),
    IOError(std::io::Error),
    RegexError(regex::Error),
    EnvVarError(std::env::VarError),
    ZipExtractionError(String),
    ConfigError(String),
    SerdeError(serde_json::Error),
    JavaVersionNotFound,
    JavaAlreadyExists,
    InvalidJavaPackage,
    MinecraftVersionNotFound,
    FailedToSaveInstallation,
    FailedToExecuteInstallation,
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

impl From<std::io::Error> for DownloadError {
    fn from(value: std::io::Error) -> Self {
        DownloadError::IO(value)
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(value: reqwest::Error) -> Self {
        Self::DownloadError(DownloadError::from(value))
    }
}

impl From<DownloadError> for BackendError {
    fn from(value: DownloadError) -> Self {
        Self::DownloadError(value)
    }
}

impl From<std::io::Error> for BackendError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ZipError> for BackendError {
    fn from(value: ZipError) -> Self {
        Self::ZipError(value)
    }
}

impl From<regex::Error> for BackendError {
    fn from(value: regex::Error) -> Self {
        Self::RegexError(value)
    }
}

impl From<std::env::VarError> for BackendError {
    fn from(value: std::env::VarError) -> Self {
        Self::EnvVarError(value)
    }
}

impl From<serde_json::Error> for BackendError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}
