use std::{env, fs, path::PathBuf};

use crate::{ASSETS_DIR, INSTALLATIONS_DIR, LAUNCHER_DIR, LIBS_DIR};

pub fn config_launcher_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("SynthLauncher"))
            .unwrap_or_else(|_| PathBuf::from("C:\\SynthLauncher"));
    }

    #[cfg(target_os = "macos")]
    {
        return env::var("HOME")
            .map(|home| {
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("SynthLauncher")
            })
            .unwrap_or_else(|_| PathBuf::from("/usr/local/synthlauncher"));
    }

    #[cfg(target_os = "linux")]
    {
        return env::var("HOME")
            .map(|home| PathBuf::from(home).join(".synthlauncher"))
            .unwrap_or_else(|_| PathBuf::from("/usr/local/synthlauncher"));
    }
}

pub fn init_launcher_dir() {
    fs::create_dir(&(*LAUNCHER_DIR)).unwrap();
    fs::create_dir_all(&(*LIBS_DIR)).unwrap();
    fs::create_dir_all(&(*ASSETS_DIR)).unwrap();
    fs::create_dir_all(&(*INSTALLATIONS_DIR)).unwrap();
}

