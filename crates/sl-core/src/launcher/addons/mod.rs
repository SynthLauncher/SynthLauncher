use sl_utils::{utils::errors::BackendError};

use crate::ADDONS_DIR;

pub fn get_addons() -> Result<Vec<String>, BackendError> {
    let mut addons = Vec::new();
    for entry in std::fs::read_dir(ADDONS_DIR.as_path())? {
        let entry = entry?;
        let path = entry.path();
        let addon = std::fs::read_to_string(path)?;

        addons.push(addon);
    }

    Ok(addons)
}
