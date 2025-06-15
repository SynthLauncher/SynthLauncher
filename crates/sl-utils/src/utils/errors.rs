use synrinth::errors::SynrinthError;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Request timed out")]
    Timeout,
    #[error("Invalid URL")]
    InvalidURL,
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("HTTP request failed with status code: {0}")]
    Status(reqwest::StatusCode),
    #[error("Invalid Header Value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
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
    #[error("An unknown error occurred: {0}")]
    OtherInstallationError(String),
    #[error("{0}")]
    Forge(#[from] ForgeInstallerErr),
}

#[derive(Debug, Error)]
pub enum ForgeInstallerErr {
    #[error("Error while downloading forge: `{0}`")]
    Download(#[from] HttpError),
    #[error("Error while compiling using javac\nstdout:\n{stdout}\nstderr:\n{stderr}")]
    CompileErr { stdout: String, stderr: String },
    #[error("Error while running installer using java\nstdout:\n{stdout}\nstderr:\n{stderr}")]
    JavaRunErr { stdout: String, stderr: String },
    #[error("Forge Installation failed, more details: {0}")]
    IOErr(#[from] std::io::Error),
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
pub enum ZipExtractionError {
    #[error("Unsupported File Extension: {0}")]
    UnsupportedFileExt(String),
}

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Zip error: {0}")]
    ZipError(#[from] ZipError),
    #[error("Zip extraction error: {0}")]
    ZipExtractionError(#[from] ZipExtractionError),
    #[error("Download error: {0}")]
    HttpError(#[from] HttpError),
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Environmental variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("JSON serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Java error: {0}")]
    JavaError(#[from] JavaError),
    #[error("Installation error: {0}")]
    InstallationError(#[from] InstallationError),
    #[error("Synrinth: {0}")]
    SynrinthError(#[from] SynrinthError),
}

impl From<reqwest::Error> for HttpError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_builder() {
            HttpError::InvalidURL
        } else if value.is_timeout() {
            HttpError::Timeout
        } else if let Some(status) = value.status() {
            HttpError::Status(status)
        } else {
            HttpError::Other(value)
        }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for BackendError {
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        BackendError::HttpError(HttpError::InvalidHeaderValue(value))
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(value: reqwest::Error) -> Self {
        BackendError::HttpError(HttpError::from(value))
    }
}
