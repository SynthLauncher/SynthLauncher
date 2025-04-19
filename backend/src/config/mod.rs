pub mod app;
pub mod config;
pub mod installations;

pub const MULTI_PATH_SEPARATOR: &'static str = if cfg!(target_os = "windows") {
    ";"
} else {
    ":"
};
