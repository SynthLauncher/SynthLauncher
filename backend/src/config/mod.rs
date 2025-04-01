pub mod app;
pub mod config;
pub mod installations;
pub mod java;

pub const MULTI_PATH_SEPERATOR: &'static str = if cfg!(target_os = "windows") {
    ";"
} else {
    ":"
};
