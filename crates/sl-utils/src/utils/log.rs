#[macro_export]
macro_rules! log_generic {
    ($level:expr, $($arg:tt)*) => {{
        let now = $crate::__chrono_reexport::Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] [{}] [{}:{}] {}", $level, now, file!(), line!(), format!($($arg)*));
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