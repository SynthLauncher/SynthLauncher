use std::{fs::File, io::Write, path::PathBuf, sync::Mutex};

static LOG_FILE: Mutex<Option<File>> = Mutex::new(Option::None);

pub fn set_log_file(path: PathBuf) {
    LOG_FILE
        .lock()
        .unwrap()
        .replace(File::create(path).expect("failed to set log file"));
}

#[doc(hidden)]
pub fn _write_to_log_file(args: std::fmt::Arguments) {
    if let Ok(mut lock) = LOG_FILE.lock() {
        if let Some(ref mut file) = *lock {
            file.write_fmt(args)
                .expect("failed to write to log file :(");
        }
    }
}

#[macro_export]
macro_rules! write_to_log_file {
    ($($arg:tt)*) => {
        $crate::utils::log::_write_to_log_file(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_generic {
    ($level:expr, $($arg:tt)*) => {{
        // FIXME:
        let now = $crate::__chrono_reexport::Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] [{}] [{}:{}] {}", $level, now, file!(), line!(), format!($($arg)*));
        $crate::write_to_log_file!("[{}] [{}] [{}:{}] {}\n", $level, now, file!(), line!(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log_generic!("INFO", $($arg)*);
    }};
}

#[macro_export]
macro_rules! elog {
    ($($arg:tt)*) => {{
        $crate::log_generic!("ERROR", $($arg)*);
    }};
}

#[macro_export]
macro_rules! wlog {
    ($($arg:tt)*) => {{
        $crate::log_generic!("WARNING", $($arg)*);
    }};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dlog {
    ($($arg:tt)*) => {{
        $crate::log_generic!("DEBUG", $($arg)*);
    }};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dlog {
    ($($arg:tt)*) => {};
}
