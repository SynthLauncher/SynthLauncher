use std::{env, io};

use zip::result::ZipError;

use super::download::DownloadErr;

#[derive(Debug)]
pub enum BackendError {
    ZipError(ZipError),
    DownlaodError(DownloadErr),
    IOError(io::Error),
    RegexError(regex::Error),
    EnvVarError(env::VarError),
    JavaVersionNotFound,
    JavaAlreadyExists,
    InvalidJavaPackage,
    MinecraftVersionNotFound,
    ZipExtractionError(String),
    ConfigError(String),
    FailedToSaveInstallations,
    VersionAlreadyInstalled,
}

impl From<reqwest::Error> for BackendError {
    fn from(value: reqwest::Error) -> Self {
        Self::DownlaodError(DownloadErr::from(value))
    }
}

impl From<DownloadErr> for BackendError {
    fn from(value: DownloadErr) -> Self {
        Self::DownlaodError(value)
    }
}

impl From<io::Error> for BackendError {
    fn from(value: io::Error) -> Self {
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

impl From<env::VarError> for BackendError {
    fn from(value: env::VarError) -> Self {
        Self::EnvVarError(value)
    }
}
